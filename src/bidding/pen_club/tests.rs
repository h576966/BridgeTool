use super::*;
use crate::bidding::agreements::Agreements;
use crate::bidding::american::american_book;
use crate::bidding::array::Logits;
use crate::bidding::bridge_tool::{Opening, OpeningSelection, select_opening};
use crate::bidding::context::Context;
use crate::bidding::inference::{Range, Relative};
use crate::bidding::trie::Trie;
use crate::bidding::{Alert, Bidder, Partnership, Table};
use contract_bridge::auction::{Auction, Call, RelativeVulnerability};
use contract_bridge::deck::fill_deals;
use contract_bridge::{AbsoluteVulnerability, Bid, Builder, FullDeal, Hand, Seat, Strain, Suit};
use rand::SeedableRng as _;
use rand::rngs::StdRng;

const fn call(level: u8, strain: Strain) -> Call {
    Call::Bid(Bid::new(level, strain))
}

fn hand(text: &str) -> Hand {
    let hand: Hand = text.parse().expect("valid test hand");
    assert_eq!(hand.len(), 13, "test hand must contain 13 cards: {text}");
    hand
}

fn best(system: &impl Bidder, auction: &[Call], text: &str) -> Call {
    best_vul(system, auction, text, RelativeVulnerability::NONE)
}

fn best_vul(
    system: &impl Bidder,
    auction: &[Call],
    text: &str,
    vul: RelativeVulnerability,
) -> Call {
    let logits: Logits = system
        .classify(hand(text), vul, auction)
        .expect("PEN-Club covers the decision through book or floor");
    logits
        .iter()
        .filter(|(_, weight)| weight.is_finite())
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("weights are not NaN"))
        .map(|(call, _)| call)
        .expect("at least one finite call")
}

fn authored_root_calls(text: &str) -> Vec<Call> {
    pen_club_book_default()
        .bind()
        .classify(hand(text), RelativeVulnerability::NONE, &[])
        .into_iter()
        .flat_map(|logits| {
            logits
                .iter()
                .filter_map(|(call, weight)| weight.is_finite().then_some(call))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn assert_alerted_at(trie: &Trie, auction: &[Call], calls: &[Call]) {
    let rules = trie
        .get(auction)
        .and_then(|classifier| classifier.as_rules())
        .unwrap_or_else(|| panic!("authored Rules node at {auction:?}"));
    for &call in calls {
        let rule = rules
            .rules()
            .iter()
            .find(|rule| rule.call() == call)
            .unwrap_or_else(|| panic!("{call} rule at {auction:?}"));
        assert!(rule.alert().is_some(), "{call} at {auction:?} must alert");
    }
}

fn partnership() -> Partnership {
    pen_club_default().bind()
}

fn deal_with_ns(north: &str, south: &str, seed: u64) -> FullDeal {
    let partial = Builder::new()
        .north(hand(north))
        .south(hand(south))
        .build_partial()
        .expect("disjoint North-South hands");
    fill_deals(&mut StdRng::seed_from_u64(seed), partial)
        .next()
        .expect("one completion")
}

fn opening_call(opening: Opening) -> Call {
    match opening {
        Opening::OneClub => call(1, Strain::Clubs),
        Opening::OneDiamond => call(1, Strain::Diamonds),
        Opening::OneHeart => call(1, Strain::Hearts),
        Opening::OneSpade => call(1, Strain::Spades),
        Opening::OneNotrump => call(1, Strain::Notrump),
        Opening::TwoClubs => call(2, Strain::Clubs),
        Opening::TwoDiamonds => call(2, Strain::Diamonds),
        Opening::TwoHearts => call(2, Strain::Hearts),
        Opening::TwoSpades => call(2, Strain::Spades),
        Opening::TwoNotrump => call(2, Strain::Notrump),
        Opening::ThreeClubs => call(3, Strain::Clubs),
        Opening::ThreeDiamonds => call(3, Strain::Diamonds),
        Opening::ThreeHearts => call(3, Strain::Hearts),
        Opening::ThreeSpades => call(3, Strain::Spades),
    }
}

#[test]
fn executable_openings_match_the_audit() {
    let system = partnership();
    for text in [
        "AKQJ.K32.Q42.J32",
        "AKQ2.3.KQJ2.8765",
        "32.AKQ2.3.AJ8765",
        "432.32.AKQJ.AJ87",
        "32.Q3.AK2.AKQJ87",
        "AQ3.KJ2.J43.Q432",
        "A2.3.5432.AKQJ87",
        "A2.3.AKQJ87.5432",
        "32.KQJ987.432.32",
        "KQJ987.32.432.32",
        "AKQ2.AK2.KJ3.QJ3",
    ] {
        let OpeningSelection::Selected(opening) = select_opening(hand(text)) else {
            panic!("audit must select one opening for {text}");
        };
        assert_eq!(best(&system, &[], text), opening_call(opening), "{text}");
    }
}

#[test]
fn opening_priorities_are_disjoint_in_the_executable_book() {
    let system = partnership();
    for (text, expected) in [
        ("AQ32.KJ54.Q432.2", call(1, Strain::Notrump)),
        ("AKQ2.3.KQJ2.8765", call(1, Strain::Diamonds)),
        ("3.AKQ2.KQJ2.8765", call(1, Strain::Hearts)),
        ("A2.3.5432.AKQJ87", call(2, Strain::Clubs)),
        ("A2.3.AKQJ87.5432", call(2, Strain::Diamonds)),
        ("32.Q3.AK2.AKQJ87", call(1, Strain::Spades)),
        ("2.A3.KQ432.AKJ87", call(1, Strain::Clubs)),
    ] {
        assert_eq!(best(&system, &[], text), expected, "{text}");
    }
}

#[test]
fn opening_hcp_boundaries_use_raw_hcp() {
    let system = partnership();
    for (text, expected) in [
        ("432.32.AKQ2.J876", call(1, Strain::Spades)),
        ("432.32.AKQJ.AJ87", call(1, Strain::Spades)),
        ("432.32.AKQJ.KQJ8", call(1, Strain::Clubs)),
        ("32.KQJ987.432.32", call(2, Strain::Hearts)),
        ("A2.3.5432.AKQJ87", call(2, Strain::Clubs)),
    ] {
        assert_eq!(best(&system, &[], text), expected, "{text}");
    }
}

#[test]
fn weak_major_and_two_notrump_boundaries_match_the_audit() {
    for (text, expected) in [
        ("432.KJT987.432.2", None),
        ("A32.JT9876.432.2", None),
        ("A32.QT9876.432.2", Some(Opening::TwoHearts)),
        ("A32.KQ9876.432.2", Some(Opening::TwoHearts)),
        ("A32.KQJ987.432.2", Some(Opening::OneHeart)),
        ("KJT987.432.432.2", None),
        ("JT9876.A32.432.2", None),
        ("QT9876.A32.432.2", Some(Opening::TwoSpades)),
        ("KQ9876.A32.432.2", Some(Opening::TwoSpades)),
        ("KQJ987.A32.432.2", Some(Opening::OneDiamond)),
        ("AKQ2.AK2.QJ3.Q32", Some(Opening::OneClub)),
        ("AKQ2.AK2.KJ3.Q32", Some(Opening::TwoNotrump)),
        ("AKQ2.AK2.KQ3.QJ2", Some(Opening::TwoNotrump)),
        ("AKQ2.AKQ.KJ3.QJ2", Some(Opening::OneClub)),
    ] {
        let audit = select_opening(hand(text));
        let expected_selection =
            expected.map_or(OpeningSelection::NoMatch, OpeningSelection::Selected);
        assert_eq!(audit, expected_selection, "audit boundary for {text}");
        assert_eq!(
            authored_root_calls(text),
            expected.into_iter().map(opening_call).collect::<Vec<_>>(),
            "authored root boundary for {text}",
        );
    }
}

#[test]
fn preempts_use_the_agreed_suit_quality_and_unfavorable_floor() {
    let system = partnership();
    for (text, expected) in [
        ("A32.QT9876.432.2", call(2, Strain::Hearts)),
        ("A32.JT9876.432.2", Call::Pass),
        ("32.QT98765.A32.2", call(3, Strain::Hearts)),
        ("32.JT98765.A32.2", Call::Pass),
        ("32.2.A32.QT98765", call(3, Strain::Clubs)),
    ] {
        assert_eq!(best(&system, &[], text), expected, "normal: {text}");
    }

    let unfavorable = RelativeVulnerability::WE;
    for (text, expected) in [
        ("A32.K98765.432.2", call(2, Strain::Hearts)),
        ("A32.QJ9876.432.2", call(2, Strain::Hearts)),
        ("K32.QT9876.J32.J", call(2, Strain::Hearts)),
        ("A32.Q98765.J32.2", Call::Pass),
        ("K32.K98765.432.2", Call::Pass),
        ("Q2.KQ98765.432.2", call(3, Strain::Hearts)),
        ("32.Q987654.AJ2.2", Call::Pass),
    ] {
        assert_eq!(
            best_vul(&system, &[], text, unfavorable),
            expected,
            "unfavorable: {text}",
        );
    }

    let their_american = american_book(&Agreements::default()).bind();
    let defense = partnership().with_opponents(&their_american);
    assert_eq!(
        best(&defense, &[call(1, Strain::Clubs)], "32.Q98765.K32.32"),
        call(2, Strain::Hearts),
        "the normal weak-jump gate accepts a queen-high suit",
    );
    for (text, expected) in [
        ("Q2.K98765.J2.J32", call(2, Strain::Hearts)),
        ("Q2.K98765.J2.432", Call::Pass),
        ("Q2.Q98765.J2.Q32", Call::Pass),
    ] {
        assert_eq!(
            best_vul(&defense, &[call(1, Strain::Clubs)], text, unfavorable,),
            expected,
            "unfavorable weak jump: {text}",
        );
    }
}

#[test]
fn unmatched_pen_opening_hands_pass_in_every_seat() {
    let system = partnership();
    for text in [
        "QJ6.65.K6.JT9832", // the reported seven-HCP false 1♣
        "QJ6.65.JT9832.K6", // the same leak through a natural 1♦ fallback
        "QJ983.65.K6.JT83", // the same leak through a natural 1♠ fallback
    ] {
        assert_eq!(select_opening(hand(text)), OpeningSelection::NoMatch);
        for leading_passes in 0..=3 {
            let auction = vec![Call::Pass; leading_passes];
            assert_eq!(best(&system, &auction, text), Call::Pass, "{text}");
        }
    }

    assert_eq!(
        best(&system, &[Call::Pass, Call::Pass], "K9.AKQ42.AJ73.A4",),
        call(1, Strain::Clubs),
        "the reported South hand must make PEN's strong artificial opening",
    );
}

#[test]
fn artificial_openings_project_their_meanings() {
    let system = partnership();

    let one_club = system.infer(RelativeVulnerability::NONE, &[call(1, Strain::Clubs)]);
    assert_eq!(one_club.rho().strength.hcp.min, 16);
    assert_eq!(one_club.rho().length(Suit::Clubs).min, 0);

    let one_diamond = system.infer(RelativeVulnerability::NONE, &[call(1, Strain::Diamonds)]);
    assert_eq!(one_diamond.rho().strength.hcp, Range::new(10, 15));
    assert_eq!(one_diamond.rho().length(Suit::Spades).min, 4);
    assert_eq!(one_diamond.rho().length(Suit::Diamonds).min, 0);

    let one_spade = system.infer(RelativeVulnerability::NONE, &[call(1, Strain::Spades)]);
    assert_eq!(one_spade.rho().length(Suit::Clubs).min, 4);
    assert_eq!(one_spade.rho().length(Suit::Spades).min, 0);

    let one_nt = system.infer(RelativeVulnerability::NONE, &[call(1, Strain::Notrump)]);
    assert_eq!(one_nt.rho().strength.hcp, Range::new(12, 15));
    assert_eq!(one_nt.rho().length(Suit::Hearts).min, 3);
    assert_eq!(one_nt.rho().length(Suit::Spades).min, 3);
}

#[test]
fn transfers_project_the_target_suit_not_the_bid_suit() {
    let system = partnership();
    for (auction, target, bid_suit) in [
        (
            vec![
                call(1, Strain::Spades),
                Call::Pass,
                call(2, Strain::Diamonds),
            ],
            Suit::Hearts,
            Suit::Diamonds,
        ),
        (
            vec![call(1, Strain::Spades), Call::Pass, call(2, Strain::Hearts)],
            Suit::Spades,
            Suit::Hearts,
        ),
        (
            vec![
                call(1, Strain::Notrump),
                Call::Pass,
                call(2, Strain::Diamonds),
            ],
            Suit::Hearts,
            Suit::Diamonds,
        ),
        (
            vec![
                call(1, Strain::Notrump),
                Call::Pass,
                call(2, Strain::Spades),
            ],
            Suit::Clubs,
            Suit::Spades,
        ),
        (
            vec![
                call(1, Strain::Notrump),
                Call::Pass,
                call(2, Strain::Notrump),
            ],
            Suit::Diamonds,
            Suit::Spades,
        ),
    ] {
        let inf = system.infer(RelativeVulnerability::NONE, &auction);
        assert!(inf.rho().length(target).min >= 5, "{auction:?}");
        assert_eq!(inf.rho().length(bid_suit).min, 0, "{auction:?}");
    }
}

#[test]
fn declared_opponents_read_pen_openings_and_transfers() {
    let their_pen = pen_club_default().bind();
    let ours = american_book(&Agreements::default())
        .bind()
        .with_opponents(&their_pen);

    let opening = ours.infer(
        RelativeVulnerability::NONE,
        &[call(1, Strain::Diamonds), Call::Pass, Call::Pass],
    );
    assert!(opening.get(Relative::Lho).length(Suit::Spades).min >= 4);
    assert_eq!(opening.get(Relative::Lho).length(Suit::Diamonds).min, 0);

    let transfer = ours.infer(
        RelativeVulnerability::NONE,
        &[
            call(1, Strain::Spades),
            Call::Pass,
            call(2, Strain::Diamonds),
        ],
    );
    assert!(transfer.get(Relative::Rho).length(Suit::Hearts).min >= 5);
    assert_eq!(transfer.get(Relative::Rho).length(Suit::Diamonds).min, 0);
}

#[test]
fn strong_club_defense_is_simple_and_readable() {
    let system = partnership();
    let one_club = call(1, Strain::Clubs);
    let pass = Call::Pass;

    assert_eq!(
        best(&system, &[one_club], "K9.AKQ42.AJ73.A4"),
        call(1, Strain::Hearts),
        "the reported 21-count must act naturally over the artificial club",
    );
    assert_eq!(
        best(&system, &[one_club], "32.2.AKQ87.AQJ87"),
        Call::Double,
        "Double shows both minors",
    );
    assert_eq!(
        best(&system, &[one_club], "KJ987.QJ987.A2.2"),
        call(1, Strain::Notrump),
        "1NT shows both majors",
    );
    assert_eq!(
        best(&system, &[one_club], "32.32.32.AKQJ876"),
        call(2, Strain::Clubs),
        "2C is the natural club single-suiter",
    );

    assert_eq!(
        best(&system, &[one_club, Call::Double, pass], "32.32.KJ87.QJ987",),
        call(2, Strain::Clubs),
    );
    assert_eq!(
        best(
            &system,
            &[one_club, call(1, Strain::Notrump), pass],
            "KJ87.QJ2.5432.32",
        ),
        call(2, Strain::Spades),
    );
    assert_eq!(
        best(
            &system,
            &[
                one_club,
                call(1, Strain::Notrump),
                pass,
                call(2, Strain::Spades),
                pass,
            ],
            "KJ987.QJ987.A2.2",
        ),
        pass,
    );

    let book = pen_club_book_default();
    let rules = book
        .defensive
        .get(&[one_club])
        .and_then(|classifier| classifier.as_rules())
        .expect("strong-club defensive rules");
    let auction = [one_club];
    let context = Context::new(RelativeVulnerability::NONE, &auction);
    let projection = |call, alert| {
        rules
            .rules()
            .iter()
            .find(|rule| rule.call() == call && rule.alert() == Some(alert))
            .expect("alerted strong-club rule")
            .project(&context)
    };
    let minors = projection(Call::Double, Alert("pen:strong-club-defense-minors"));
    assert!(minors.length(Suit::Clubs).min >= 5);
    assert!(minors.length(Suit::Diamonds).min >= 5);
    let majors = projection(
        call(1, Strain::Notrump),
        Alert("pen:strong-club-defense-majors"),
    );
    assert!(majors.length(Suit::Hearts).min >= 5);
    assert!(majors.length(Suit::Spades).min >= 5);
}

#[test]
fn strong_hands_still_double_disclosed_natural_openings() {
    let their_american = american_book(&Agreements::default()).bind();
    let ours = partnership().with_opponents(&their_american);
    assert_eq!(
        best(&ours, &[call(1, Strain::Clubs)], "AQ3.KJ2.AJ43.Q32",),
        Call::Double,
        "17+ doubles even without classic takeout shape",
    );
}

#[test]
fn natural_major_overcall_beats_a_misshapen_minor_takeout_double() {
    let their_american = american_book(&Agreements::default()).bind();
    let ours = partnership().with_opponents(&their_american);

    assert_eq!(
        best(&ours, &[call(1, Strain::Clubs)], "KQ.KQT94.KJ943.3",),
        call(1, Strain::Hearts),
        "a 5-5 hand with only two spades overcalls its five-card major",
    );
    assert_eq!(
        best(&ours, &[call(1, Strain::Clubs)], "KQ32.AJ3.KJ942.2",),
        Call::Double,
        "an ordinary takeout double retains four-three majors",
    );
}

#[test]
fn artificial_calls_are_explicitly_alerted() {
    let system = pen_club_book_default();
    let pass = Call::Pass;
    let c = &system.constructive.0;
    for (auction, calls) in [
        (
            vec![],
            vec![
                call(1, Strain::Clubs),
                call(1, Strain::Diamonds),
                call(1, Strain::Spades),
            ],
        ),
        (
            vec![call(1, Strain::Clubs), pass],
            vec![
                call(1, Strain::Diamonds),
                call(1, Strain::Hearts),
                call(1, Strain::Spades),
                call(2, Strain::Clubs),
                call(2, Strain::Diamonds),
                call(3, Strain::Clubs),
            ],
        ),
        (
            vec![call(1, Strain::Diamonds), pass],
            vec![
                call(3, Strain::Notrump),
                call(4, Strain::Clubs),
                call(4, Strain::Diamonds),
                call(4, Strain::Hearts),
                call(4, Strain::Notrump),
            ],
        ),
        (
            vec![call(1, Strain::Hearts), pass],
            vec![
                call(1, Strain::Spades),
                call(1, Strain::Notrump),
                call(2, Strain::Clubs),
                call(2, Strain::Diamonds),
                call(2, Strain::Notrump),
                call(3, Strain::Notrump),
                call(4, Strain::Clubs),
                call(4, Strain::Diamonds),
                call(4, Strain::Notrump),
            ],
        ),
        (
            vec![call(1, Strain::Spades), pass],
            vec![
                call(1, Strain::Notrump),
                call(2, Strain::Clubs),
                call(2, Strain::Diamonds),
                call(2, Strain::Hearts),
                call(2, Strain::Spades),
                call(2, Strain::Notrump),
            ],
        ),
        (
            vec![call(1, Strain::Notrump), pass],
            vec![
                call(2, Strain::Clubs),
                call(2, Strain::Diamonds),
                call(2, Strain::Hearts),
                call(2, Strain::Spades),
                call(2, Strain::Notrump),
                call(4, Strain::Diamonds),
                call(4, Strain::Hearts),
                call(4, Strain::Notrump),
            ],
        ),
        (
            vec![call(1, Strain::Notrump), pass, call(2, Strain::Clubs), pass],
            vec![call(2, Strain::Diamonds)],
        ),
    ] {
        assert_alerted_at(c, &auction, &calls);
    }

    for (auction, calls) in [
        (
            vec![call(1, Strain::Spades), Call::Double],
            vec![
                call(1, Strain::Notrump),
                call(2, Strain::Clubs),
                call(2, Strain::Diamonds),
                call(2, Strain::Hearts),
            ],
        ),
        (
            vec![
                call(1, Strain::Spades),
                pass,
                call(2, Strain::Diamonds),
                Call::Double,
            ],
            vec![call(2, Strain::Hearts)],
        ),
        (
            vec![
                call(1, Strain::Notrump),
                pass,
                call(2, Strain::Spades),
                Call::Double,
            ],
            vec![call(3, Strain::Clubs)],
        ),
        (
            vec![call(2, Strain::Diamonds), Call::Double],
            vec![Call::Redouble, call(2, Strain::Hearts)],
        ),
    ] {
        assert_alerted_at(&system.competitive.0, &auction, &calls);
    }
}

#[test]
fn authored_uncontested_sequences_bid_as_pen_club() {
    let system = partnership();
    let pass = Call::Pass;

    let one_club = call(1, Strain::Clubs);
    let one_diamond = call(1, Strain::Diamonds);
    assert_eq!(
        best(&system, &[one_club, pass], "432.432.432.5432"),
        one_diamond
    );
    assert_eq!(
        best(
            &system,
            &[one_club, pass, one_diamond, pass],
            "AKQJ2.K32.Q3.K32",
        ),
        call(1, Strain::Spades),
    );

    let one_heart = call(1, Strain::Hearts);
    let relay = call(1, Strain::Spades);
    assert_eq!(best(&system, &[one_heart, pass], "432.432.432.5432"), relay);
    assert_eq!(
        best(&system, &[one_heart, pass, relay, pass], "32.AKQJ2.A432.32",),
        call(1, Strain::Notrump),
    );

    let two_diamonds = call(2, Strain::Diamonds);
    let two_hearts = call(2, Strain::Hearts);
    assert_eq!(
        best(&system, &[two_diamonds, pass], "K32.QJ987.432.32"),
        two_hearts,
    );
    assert_eq!(
        best(
            &system,
            &[two_diamonds, pass, two_hearts, pass],
            "A2.3.AKQJ87.5432",
        ),
        call(2, Strain::Notrump),
    );
}

#[test]
fn one_diamond_finds_spades_after_a_low_overcall() {
    let system = partnership();
    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Diamonds), call(1, Strain::Hearts)],
            "J32.A32.432.K432",
        ),
        call(1, Strain::Spades),
    );
}

#[test]
fn one_diamond_raises_spades_with_distribution_after_one_notrump() {
    let their_american = american_book(&Agreements::default()).bind();
    let system = partnership().with_opponents(&their_american);

    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Diamonds), call(1, Strain::Notrump),],
            "Q862..J652.JT975",
        ),
        call(2, Strain::Spades),
        "four HCP plus a side-suit void is seven support points in the known fit",
    );
}

#[test]
fn one_club_negative_transfers_cover_every_natural_one_level_overcall() {
    let system = partnership();
    let one_club = call(1, Strain::Clubs);
    for (overcall, text, expected) in [
        (call(1, Strain::Diamonds), "432.QJ987.32.432", Call::Double),
        (
            call(1, Strain::Diamonds),
            "QJ987.432.32.432",
            call(1, Strain::Hearts),
        ),
        (
            call(1, Strain::Diamonds),
            "432.432.32.QJ987",
            call(1, Strain::Spades),
        ),
        (call(1, Strain::Hearts), "QJ987.432.32.432", Call::Double),
        (
            call(1, Strain::Hearts),
            "432.432.32.QJ987",
            call(1, Strain::Spades),
        ),
        (
            call(1, Strain::Hearts),
            "432.432.QJ987.32",
            call(1, Strain::Notrump),
        ),
        (call(1, Strain::Spades), "432.QJ987.32.432", Call::Double),
        (
            call(1, Strain::Spades),
            "432.432.32.QJ987",
            call(1, Strain::Notrump),
        ),
        (
            call(1, Strain::Spades),
            "432.432.QJ987.32",
            call(2, Strain::Clubs),
        ),
    ] {
        assert_eq!(
            best(&system, &[one_club, overcall], text),
            expected,
            "{text}"
        );
    }

    assert_eq!(
        best(
            &system,
            &[one_club, call(1, Strain::Spades)],
            "432.432.432.5432",
        ),
        Call::Pass,
        "a negative without a five-card unbid suit may still pass",
    );
}

#[test]
fn one_club_reopening_and_transfer_rebids_are_descriptive() {
    let p = Call::Pass;
    let one_club = call(1, Strain::Clubs);
    let one_spade = call(1, Strain::Spades);
    let their_american = american_book(&Agreements::default()).bind();
    let system = partnership().with_opponents(&their_american);

    let start = [p, p, one_club, one_spade];
    assert_eq!(
        best(&system, &start, "96532.J8763.A7.J"),
        Call::Double,
        "the reported North hand shows its five hearts",
    );
    assert_eq!(
        best(
            &system,
            &[p, p, one_club, one_spade, Call::Double, p],
            "2.AKQ.KJ96432.A8",
        ),
        call(2, Strain::Diamonds),
        "opener may decline the heart transfer to show long diamonds",
    );
    assert_eq!(
        best(
            &system,
            &[p, p, one_club, one_spade, p, p],
            "2.AKQ.KJ96432.A8",
        ),
        call(2, Strain::Diamonds),
        "a natural one-level overcall may not be passed out",
    );

    assert_eq!(
        best(&system, &[one_club, one_spade, p, p], "AQ2.AK32.KQ3.J32",),
        call(1, Strain::Notrump),
    );
    assert_eq!(
        best(&system, &[one_club, one_spade, p, p], "32.AKQ2.KQJ3.A32",),
        Call::Double,
    );
    assert_eq!(
        best(&system, &[one_club, one_spade, p, p], "AKQ2.AQ32.Q32.K2",),
        call(2, Strain::Clubs),
        "2C is the artificial 20+ relay",
    );
}

#[test]
fn one_club_interference_transfers_are_alerted_and_readable() {
    let system = partnership();
    let one_club = call(1, Strain::Clubs);
    let one_spade = call(1, Strain::Spades);
    let inf = system.infer(
        RelativeVulnerability::NONE,
        &[one_club, one_spade, Call::Double],
    );
    assert!(inf.rho().length(Suit::Hearts).min >= 5);
    assert!(inf.rho().strength.hcp.max <= 8);

    let strong = system.infer(
        RelativeVulnerability::NONE,
        &[
            one_club,
            one_spade,
            Call::Pass,
            Call::Pass,
            call(2, Strain::Clubs),
        ],
    );
    assert!(strong.rho().strength.hcp.min >= 20);

    let book = pen_club_book_default();
    assert_alerted_at(
        &book.competitive.0,
        &[one_club, one_spade],
        &[
            Call::Double,
            call(1, Strain::Notrump),
            call(2, Strain::Clubs),
            call(2, Strain::Spades),
        ],
    );
    assert_alerted_at(
        &book.competitive.0,
        &[one_club, one_spade, Call::Pass, Call::Pass],
        &[Call::Double, call(2, Strain::Clubs)],
    );
    assert_alerted_at(
        &book.competitive.0,
        &[one_club, one_spade, Call::Double, Call::Pass],
        &[call(2, Strain::Hearts)],
    );
}

#[test]
fn direct_three_notrump_shows_exactly_three_card_support() {
    let system = partnership();
    let pass = Call::Pass;
    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Diamonds), pass],
            "KQ3.AT3.KJ32.Q32",
        ),
        call(3, Strain::Notrump),
    );
    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Hearts), pass],
            "KQ3.AT3.KJ32.Q32",
        ),
        call(3, Strain::Notrump),
    );
}

#[test]
fn one_spade_preferences_and_transfers_are_playable() {
    let system = partnership();
    let one_spade = call(1, Strain::Spades);
    let pass = Call::Pass;
    assert_eq!(
        best(&system, &[one_spade, pass], "432.32.KQJ8.5432"),
        call(1, Strain::Notrump),
    );
    assert_eq!(
        best(&system, &[one_spade, pass], "32.KQJ87.432.432"),
        call(2, Strain::Diamonds),
    );
    assert_eq!(
        best(
            &system,
            &[one_spade, pass, call(2, Strain::Diamonds), pass],
            "432.32.AKQJ.AJ87",
        ),
        call(2, Strain::Hearts),
    );
    assert_eq!(
        best(
            &system,
            &[one_spade, pass, call(2, Strain::Diamonds), Call::Double],
            "432.32.AKQJ.AJ87",
        ),
        call(2, Strain::Hearts),
    );
}

#[test]
fn one_spade_asks_disclose_both_branches_and_transfers_break_exactly() {
    let system = partnership();
    let one_spade = call(1, Strain::Spades);
    let pass = Call::Pass;

    assert_eq!(
        best(&system, &[one_spade, pass], "KQ3.AJ2.Q43.J432"),
        call(2, Strain::Spades),
    );
    assert_eq!(
        best(&system, &[one_spade, pass], "KQ3.AT2.Q43.KJ32"),
        call(2, Strain::Notrump),
    );

    let general_ask = [one_spade, pass, call(2, Strain::Spades), pass];
    for (text, expected) in [
        ("432.32.AKQJ.AJ87", call(2, Strain::Notrump)),
        ("32.32.AKQJ.AJ987", call(3, Strain::Clubs)),
        ("32.AKQ2.3.AKQJ87", call(3, Strain::Hearts)),
        ("AKQ2.32.3.AKQJ87", call(3, Strain::Spades)),
        ("32.32.AKQ2.AKQJ8", call(3, Strain::Diamonds)),
        ("AQ3.K32.32.AKQJ8", call(3, Strain::Notrump)),
    ] {
        assert_eq!(best(&system, &general_ask, text), expected, "{text}");
    }

    let heart_transfer = [one_spade, pass, call(2, Strain::Diamonds), pass];
    assert_eq!(
        best(&system, &heart_transfer, "KQ.A32.KJ87.QT32"),
        call(3, Strain::Hearts),
    );
    assert_eq!(
        best(&system, &heart_transfer, "KQ.A32.QJ87.JT32"),
        call(2, Strain::Hearts),
    );
    assert_eq!(
        best(&system, &heart_transfer, "AQ3.K32.2.AKQJ87"),
        call(2, Strain::Notrump),
    );
}

#[test]
fn one_notrump_stayman_and_transfers_complete() {
    let system = partnership();
    let one_nt = call(1, Strain::Notrump);
    let pass = Call::Pass;

    assert_eq!(
        best(&system, &[one_nt, pass], "K32.QJ87.A32.432"),
        call(2, Strain::Clubs),
    );
    assert_eq!(
        best(
            &system,
            &[one_nt, pass, call(2, Strain::Clubs), pass],
            "AQ3.KJ2.J43.Q432",
        ),
        call(2, Strain::Diamonds),
    );

    assert_eq!(
        best(&system, &[one_nt, pass], "KQJ87.32.432.432"),
        call(2, Strain::Hearts),
    );
    assert_eq!(
        best(
            &system,
            &[one_nt, pass, call(2, Strain::Hearts), pass],
            "AQ3.KJ2.J43.Q432",
        ),
        call(2, Strain::Spades),
    );

    assert_eq!(
        best(&system, &[one_nt, pass], "32.32.432.AKQJ87"),
        call(3, Strain::Clubs),
    );
    assert_eq!(
        best(
            &system,
            &[one_nt, pass, call(2, Strain::Spades), pass],
            "AQ3.KJ2.J43.Q432",
        ),
        call(3, Strain::Clubs),
    );
}

#[test]
fn one_notrump_answers_superaccepts_texas_and_quantitative_are_exact() {
    let system = partnership();
    let one_nt = call(1, Strain::Notrump);
    let pass = Call::Pass;
    let stayman = [one_nt, pass, call(2, Strain::Clubs), pass];

    assert_eq!(
        best(&system, &stayman, "AQ3.KJ42.J43.Q32"),
        call(2, Strain::Hearts),
    );
    assert_eq!(
        best(&system, &stayman, "KJ42.AQ3.J43.Q32"),
        call(2, Strain::Spades),
    );

    let heart_transfer = [one_nt, pass, call(2, Strain::Diamonds), pass];
    assert_eq!(
        best(&system, &heart_transfer, "KQ3.AJ42.K43.Q32"),
        call(3, Strain::Hearts),
    );
    assert_eq!(
        best(&system, &heart_transfer, "KQ3.AJ42.Q43.Q32"),
        call(2, Strain::Hearts),
    );

    assert_eq!(
        best(&system, &[one_nt, pass], "A32.KQJ987.32.32"),
        call(4, Strain::Diamonds),
    );
    assert_eq!(
        best(&system, &[one_nt, pass], "AQ3.KJ2.KQ3.KJ32"),
        call(4, Strain::Notrump),
    );
    assert_eq!(
        best(
            &system,
            &[one_nt, pass, call(4, Strain::Notrump), pass],
            "KQ3.AJ42.K43.Q32",
        ),
        call(6, Strain::Notrump),
    );
    assert_eq!(
        best(
            &system,
            &[one_nt, pass, call(4, Strain::Notrump), pass],
            "KQ3.AJ42.Q43.Q32",
        ),
        Call::Pass,
    );
}

#[test]
fn natural_two_major_overcalls_get_only_the_provisional_natural_policy() {
    let system = partnership();
    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Notrump), call(2, Strain::Hearts)],
            "A32.KQJ2.432.432",
        ),
        Call::Double,
    );
    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Notrump), call(2, Strain::Spades)],
            "32.KQJ2.432.5432",
        ),
        Call::Pass,
    );
}

#[test]
fn exact_five_two_three_three_opens_one_diamond() {
    let text = "AKQJ2.32.432.432";
    assert_eq!(
        select_opening(hand(text)),
        OpeningSelection::Selected(Opening::OneDiamond),
    );
    assert_eq!(best(&partnership(), &[], text), call(1, Strain::Diamonds));
}

#[test]
fn thirteen_point_minimum_and_maximum_split_uses_shape() {
    let system = partnership();
    let pass = Call::Pass;

    assert_eq!(
        best(
            &system,
            &[
                call(2, Strain::Diamonds),
                pass,
                call(2, Strain::Hearts),
                pass
            ],
            "KQ2.A32.KJ9876.2",
        ),
        call(3, Strain::Hearts),
        "3-3-6-1 has only nine cards in its two longest suits",
    );
    assert_eq!(
        best(
            &system,
            &[
                call(2, Strain::Diamonds),
                pass,
                call(2, Strain::Hearts),
                pass
            ],
            "KQ.A32.KJ98765.2",
        ),
        call(2, Strain::Spades),
        "2-3-7-1 has ten cards in its two longest suits",
    );

    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Hearts), pass, call(1, Strain::Spades), pass],
            "KQ2.AJ432.QJ32.2",
        ),
        call(1, Strain::Notrump),
    );
    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Hearts), pass, call(1, Strain::Spades), pass],
            "KQ.AJ5432.QJ32.2",
        ),
        call(2, Strain::Hearts),
    );

    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Spades),
                pass,
                call(1, Strain::Notrump),
                pass
            ],
            "KQ.A2.QJ987.J432",
        ),
        call(2, Strain::Diamonds),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Spades),
                pass,
                call(1, Strain::Notrump),
                pass
            ],
            "KQ.A.J9876.QJ432",
        ),
        call(3, Strain::Diamonds),
    );
}

#[test]
fn pen_fallback_respects_game_force_and_slam_ceiling() {
    let system = partnership();
    let pass = Call::Pass;
    let gf = [
        call(1, Strain::Clubs),
        pass,
        call(1, Strain::Hearts),
        pass,
        call(1, Strain::Notrump),
        pass,
    ];
    let logits = system
        .classify(hand("KQJ87.A32.432.32"), RelativeVulnerability::NONE, &gf)
        .expect("PEN-safe fallback covers the GF continuation");
    assert_eq!(logits.0[Call::Pass], f32::NEG_INFINITY);

    let off_book = [
        call(1, Strain::Diamonds),
        pass,
        call(1, Strain::Notrump),
        pass,
        call(2, Strain::Clubs),
        pass,
    ];
    let logits = system
        .classify(
            hand("AKQJ2.32.432.432"),
            RelativeVulnerability::NONE,
            &off_book,
        )
        .expect("PEN-safe fallback covers an unspecified natural continuation");
    assert_eq!(logits.0[call(4, Strain::Notrump)], f32::NEG_INFINITY,);
    assert_eq!(logits.0[Call::Double], f32::NEG_INFINITY);
    assert_eq!(logits.0[Call::Redouble], f32::NEG_INFINITY);
    for level in 1..=7 {
        for strain in Strain::ASC {
            let candidate = Bid::new(level, strain);
            let above_game = match strain {
                Strain::Clubs | Strain::Diamonds => level > 5,
                Strain::Hearts | Strain::Spades => level > 4,
                Strain::Notrump => level > 3,
            };
            if above_game {
                assert_eq!(logits.0[Call::Bid(candidate)], f32::NEG_INFINITY);
            }
        }
    }
}

#[test]
fn pen_fallback_uses_combined_hcp_for_a_no_fit_invitation() {
    let system = partnership();
    let pass = Call::Pass;
    let invitation = [
        call(1, Strain::Notrump),
        pass,
        call(2, Strain::Diamonds),
        pass,
        call(2, Strain::Hearts),
        pass,
        call(2, Strain::Notrump),
        pass,
    ];

    let minimum = system
        .classify(
            hand("KQ3.AJ2.Q43.J432"),
            RelativeVulnerability::NONE,
            &invitation,
        )
        .expect("the fallback covers an invitation");
    assert!(minimum.0[Call::Pass].is_finite());

    let maximum = system
        .classify(
            hand("KQ3.AT2.Q43.KJ32"),
            RelativeVulnerability::NONE,
            &invitation,
        )
        .expect("the fallback covers an invitation");
    assert_eq!(maximum.0[Call::Pass], f32::NEG_INFINITY);
}

#[test]
fn strong_club_minor_positive_uses_general_slam_search() {
    let system = partnership();
    let pass = Call::Pass;
    let north = "AK52.A763.A73.J3";
    let south = "64.K.KJ54.AQ7542";
    let start = [
        call(1, Strain::Clubs),
        pass,
        call(2, Strain::Clubs),
        pass,
        call(2, Strain::Notrump),
        pass,
    ];

    assert_eq!(
        best(&system, &start, south),
        call(3, Strain::Diamonds),
        "the four-card side suit keeps the game force below 3NT",
    );

    let after_three_diamonds = [start.as_slice(), &[call(3, Strain::Diamonds), pass]].concat();
    assert_eq!(
        best(&system, &after_three_diamonds, north),
        call(4, Strain::Clubs),
        "two clubs opposite a shown six establishes the eight-card fit",
    );

    let after_fit = [
        after_three_diamonds.as_slice(),
        &[call(4, Strain::Clubs), pass],
    ]
    .concat();
    let fit = system.infer(RelativeVulnerability::NONE, &after_fit);
    assert!(
        fit.partner().length(Suit::Clubs).min >= 2,
        "the fit-setting raise promises the two cards needed opposite six",
    );
    assert_eq!(
        best(&system, &after_fit, south),
        call(4, Strain::Diamonds),
        "with spades uncontrolled responder shows the cheapest side control",
    );
    let (_, control_rule) = system
        .explain_call(
            hand(south),
            RelativeVulnerability::NONE,
            &after_fit,
            call(4, Strain::Diamonds),
        )
        .expect("the general control fallback explains its call");
    assert_eq!(
        control_rule.and_then(|rule| rule.alert),
        Some("pen:mixed-control"),
    );

    let after_control = [after_fit.as_slice(), &[call(4, Strain::Diamonds), pass]].concat();
    let control = system.infer(RelativeVulnerability::NONE, &after_control);
    assert_eq!(
        control.control_bid(),
        Some((10, Suit::Clubs)),
        "the alerted diamond call is machine-readable as a club control sequence",
    );
    assert_eq!(
        best(&system, &after_control, north),
        call(4, Strain::Notrump),
        "opener holds the other side controls and can now ask for keycards",
    );
    let (_, keycard_rule) = system
        .explain_call(
            hand(north),
            RelativeVulnerability::NONE,
            &after_control,
            call(4, Strain::Notrump),
        )
        .expect("the general RKCB fallback explains its call");
    assert_eq!(
        keycard_rule.and_then(|rule| rule.alert),
        Some("pen:rkcb-1430"),
    );

    let after_keycard = [after_control.as_slice(), &[call(4, Strain::Notrump), pass]].concat();
    assert_eq!(
        best(&system, &after_keycard, south),
        call(5, Strain::Clubs),
        "one keycard answers RKCB 1430",
    );
}

#[test]
fn general_slam_search_keeps_the_measured_29_point_floor() {
    let system = partnership();
    let pass = Call::Pass;
    let start = [
        call(1, Strain::Clubs),
        pass,
        call(2, Strain::Clubs),
        pass,
        call(2, Strain::Notrump),
        pass,
    ];

    assert_eq!(
        best(&system, &start, "64.K.QJ54.AQ7542"),
        call(3, Strain::Notrump),
        "a twelve-count opposite the 16-point floor must not start the 29-point search",
    );
}

#[test]
fn delayed_below_game_raises_promise_the_cards_needed_for_eight() {
    let system = partnership();
    let pass = Call::Pass;
    let four_four = system.infer(
        RelativeVulnerability::NONE,
        &[
            call(1, Strain::Clubs),
            pass,
            call(1, Strain::Hearts),
            pass,
            call(2, Strain::Clubs),
            pass,
            call(2, Strain::Diamonds),
            pass,
            call(2, Strain::Hearts),
            pass,
        ],
    );
    assert!(four_four.partner().length(Suit::Hearts).min >= 4);

    let five_three = system.infer(
        RelativeVulnerability::NONE,
        &[
            call(1, Strain::Hearts),
            pass,
            call(1, Strain::Notrump),
            pass,
            call(2, Strain::Clubs),
            pass,
            call(2, Strain::Diamonds),
            pass,
            call(2, Strain::Spades),
            pass,
        ],
    );
    assert!(five_three.partner().length(Suit::Spades).min >= 3);
}

#[test]
fn rkcb_and_interference_answers_are_alerted() {
    let system = pen_club_book_default();
    let pass = Call::Pass;
    assert_alerted_at(
        &system.constructive.0,
        &[call(1, Strain::Diamonds), pass],
        &[call(4, Strain::Notrump)],
    );
    assert_alerted_at(
        &system.constructive.0,
        &[
            call(1, Strain::Clubs),
            pass,
            call(1, Strain::Hearts),
            pass,
            call(2, Strain::Hearts),
            pass,
        ],
        &[call(4, Strain::Notrump), call(5, Strain::Clubs)],
    );
    assert_alerted_at(
        &system.constructive.0,
        &[
            call(1, Strain::Diamonds),
            pass,
            call(4, Strain::Notrump),
            pass,
        ],
        &[
            call(5, Strain::Clubs),
            call(5, Strain::Diamonds),
            call(5, Strain::Hearts),
            call(5, Strain::Spades),
        ],
    );
    assert_alerted_at(
        &system.constructive.0,
        &[
            call(1, Strain::Diamonds),
            pass,
            call(4, Strain::Notrump),
            Call::Double,
        ],
        &[Call::Pass, Call::Redouble],
    );
}

#[test]
fn confirmed_natural_defenses_have_authored_actions() {
    let opponents = american_book(&Agreements::default()).bind();
    let system = partnership().with_opponents(&opponents);
    assert_eq!(
        best(&system, &[call(1, Strain::Clubs)], "KQJ2.AJ32.QJ32.2",),
        Call::Double,
    );
    assert_eq!(
        best(&system, &[call(1, Strain::Clubs)], "KQ2.AJ2.QJ32.K32",),
        call(1, Strain::Notrump),
    );
    assert_eq!(
        best(&system, &[call(1, Strain::Notrump)], "KQJ87.AJ32.32.32",),
        call(2, Strain::Clubs),
    );
    assert_eq!(
        best(&system, &[call(3, Strain::Hearts)], "KQJ2.2.AJ32.KQJ2",),
        Call::Double,
    );
}

#[test]
fn doubled_artificial_calls_keep_their_pen_continuations() {
    let system = partnership();
    let pass = Call::Pass;

    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Clubs),
                pass,
                call(1, Strain::Diamonds),
                Call::Double,
            ],
            "K9.AKQ42.AJ73.A4",
        ),
        call(1, Strain::Hearts),
    );
    assert_eq!(
        best(
            &system,
            &[call(1, Strain::Diamonds), Call::Double],
            "Q862.3.J652.JT97",
        ),
        call(1, Strain::Spades),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Diamonds),
                pass,
                call(1, Strain::Hearts),
                Call::Double,
            ],
            "AJ97.K97652.Q3.A",
        ),
        call(1, Strain::Spades),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Hearts),
                pass,
                call(1, Strain::Spades),
                Call::Double,
            ],
            "K63.AJ7543.J42.A",
        ),
        call(1, Strain::Notrump),
    );
}

#[test]
fn doubled_two_suiters_reach_an_advertised_suit() {
    let system = partnership();
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Notrump),
                call(2, Strain::Clubs),
                Call::Double,
            ],
            "32.98765.432.432",
        ),
        call(2, Strain::Hearts),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Hearts),
                call(2, Strain::Hearts),
                Call::Double,
            ],
            "32.432.432.98765",
        ),
        call(2, Strain::Spades),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Hearts),
                call(2, Strain::Notrump),
                Call::Double,
            ],
            "32.432.32.QJ9876",
        ),
        call(3, Strain::Clubs),
    );
}

#[test]
fn doubled_natural_partscores_run_only_with_a_safer_suit() {
    let system = partnership();
    let pass = Call::Pass;

    assert_eq!(
        best(
            &system,
            &[call(2, Strain::Spades), Call::Double],
            "2.KQJ87.432.5432",
        ),
        call(3, Strain::Hearts),
    );
    assert_eq!(
        best(
            &system,
            &[call(2, Strain::Spades), Call::Double, pass, pass,],
            "KT7632.QJ865.4.3",
        ),
        call(3, Strain::Hearts),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Clubs),
                call(1, Strain::Hearts),
                Call::Double,
            ],
            "QJ987.32.432.432",
        ),
        call(1, Strain::Spades),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Clubs),
                call(1, Strain::Hearts),
                Call::Double,
                pass,
                pass,
            ],
            "32.KQJ87.QJ987.3",
        ),
        call(2, Strain::Diamonds),
    );
}

#[test]
fn doubled_splinters_and_controls_cannot_become_the_contract() {
    let system = partnership();
    let pass = Call::Pass;
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Diamonds),
                pass,
                call(4, Strain::Clubs),
                Call::Double,
            ],
            "AQ95.Q.Q92.AJT84",
        ),
        call(4, Strain::Spades),
    );
    assert_eq!(
        best(
            &system,
            &[
                call(1, Strain::Diamonds),
                pass,
                call(4, Strain::Clubs),
                pass,
                call(4, Strain::Hearts),
                Call::Double,
            ],
            "K643.8432.KJ653.",
        ),
        call(4, Strain::Spades),
    );
}

#[test]
fn contested_four_major_fallback_needs_fit_and_values_or_favorable_sacrifice() {
    let system = partnership();
    let auction = [
        call(1, Strain::Diamonds),
        Call::Pass,
        call(1, Strain::Spades),
        call(2, Strain::Clubs),
        call(2, Strain::Spades),
        Call::Double,
    ];
    let low = hand("QJT92.4.J532.983");
    let blocked = system
        .classify(low, RelativeVulnerability::NONE, &auction)
        .expect("fallback covers the competitive decision");
    assert_eq!(
        blocked.0[call(4, Strain::Spades)],
        f32::NEG_INFINITY,
        "a weak nine-card fit is not enough at equal vulnerability",
    );
    let favorable = system
        .classify(low, RelativeVulnerability::THEY, &auction)
        .expect("fallback covers the favorable sacrifice");
    assert!(
        favorable.0[call(4, Strain::Spades)].is_finite(),
        "a nine-card fit may compete to game only at favorable vulnerability",
    );
}

#[test]
fn seeded_competitive_auctions_complete_through_the_table() {
    let system = pen_club_default();
    let table = Table::of_systems(&system, &system, Seat::North, AbsoluteVulnerability::NONE);

    let nt_deal = deal_with_ns("AQ3.KJ2.J43.Q432", "K42.AQ43.762.AKT", 7);
    let mut penalty = Auction::new();
    penalty.push(call(1, Strain::Notrump));
    penalty.push(call(2, Strain::Hearts));
    assert_eq!(
        table.next_call(nt_deal[Seat::South], &penalty),
        Call::Double
    );
    penalty.push(Call::Double);
    let penalty = table.bid_out_from(&nt_deal, penalty);
    assert!(penalty.has_ended());
    assert!(penalty.len() < 100);

    let transfer_deal = deal_with_ns("432.32.AKQJ.AJ87", "KT9.JT987.432.K2", 11);
    let mut doubled_transfer = Auction::new();
    doubled_transfer.push(call(1, Strain::Spades));
    doubled_transfer.push(Call::Pass);
    doubled_transfer.push(call(2, Strain::Diamonds));
    doubled_transfer.push(Call::Double);
    assert_eq!(
        table.next_call(transfer_deal[Seat::North], &doubled_transfer),
        call(2, Strain::Hearts),
    );
    let doubled_transfer = table.bid_out_from(&transfer_deal, doubled_transfer);
    assert!(doubled_transfer.has_ended());
    assert!(doubled_transfer.len() < 100);
}

#[test]
fn a_real_table_completes_a_legal_pen_club_auction() {
    const PBN: &str = "N:AK72.K65.K43.Q82 QJT.AQJ.AQJ.AKJT 986.T987.T98.976 543.432.7652.543";
    let deal: FullDeal = PBN.parse().expect("valid complete deal");
    let system = pen_club_default();
    let table = Table::of_systems(&system, &system, Seat::North, AbsoluteVulnerability::NONE);
    let auction: Auction = table.bid_out(&deal);
    assert!(auction.has_ended());
    assert!(auction.len() < 100, "auction must terminate: {auction:?}");
}
