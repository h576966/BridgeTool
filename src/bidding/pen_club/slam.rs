//! PEN-Club slam asks and their machine-readable continuations.

use crate::bidding::agreements::Agreements;
use crate::bidding::american::slam::count_keycards;
use crate::bidding::constraint::{Cons, Constraint, described, hcp, len};
use crate::bidding::context::Context;
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Hand, Rank, Strain, Suit};

pub(super) const RKCB: Alert = Alert("pen:rkcb-1430");
const QUEEN_ASK: Alert = Alert("pen:trump-queen-ask");
const KING_ASK: Alert = Alert("pen:specific-king-ask");
const VOID_RESPONSE: Alert = Alert("pen:rkcb-void-response");
const EXCLUSION: Alert = Alert("pen:exclusion-rkcb");
const ROPI: Alert = Alert("pen:ropi");
const DOPI: Alert = Alert("pen:dopi");
const DEPO: Alert = Alert("pen:depo");
const CONTROL: Alert = Alert("pen:mixed-control");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

fn keycards(trump: Suit, counts: &'static [usize]) -> Cons<impl Constraint + Clone> {
    described(
        format!(
            "{} keycards for {trump}",
            counts
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(" or ")
        ),
        move |hand: Hand, _: &Context<'_>| counts.contains(&count_keycards(hand, trump)),
    )
}

fn exclusion_keycards(
    trump: Suit,
    excluded: Suit,
    counts: &'static [usize],
) -> Cons<impl Constraint + Clone> {
    described(
        format!(
            "{} relevant keycards, excluding the {excluded} ace",
            counts
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(" or ")
        ),
        move |hand: Hand, _: &Context<'_>| {
            let aces = Suit::ASC
                .into_iter()
                .filter(|&suit| suit != excluded && hand[suit].contains(Rank::A))
                .count();
            let trump_king = usize::from(hand[trump].contains(Rank::K));
            counts.contains(&(aces + trump_king))
        },
    )
}

fn trump_queen(trump: Suit) -> Cons<impl Constraint + Clone> {
    described(
        format!("the {trump} queen or a known ten-card {trump} fit"),
        move |hand: Hand, context: &Context<'_>| {
            hand[trump].contains(Rank::Q)
                || hand[trump].len() + usize::from(context.inferences().partner().length(trump).min)
                    >= 10
        },
    )
}

pub(super) fn all_side_controls(trump: Suit) -> Cons<impl Constraint + Clone> {
    let sides: Vec<Suit> = Suit::ASC
        .into_iter()
        .filter(|&suit| suit != trump)
        .collect();
    described(
        format!("first- or second-round control in every side suit outside {trump}"),
        move |hand: Hand, context: &Context<'_>| {
            sides.iter().all(|&suit| {
                let holding = hand[suit];
                holding.contains(Rank::A)
                    || holding.contains(Rank::K)
                    || (holding.len() <= 1 && context.inferences().partner().length(suit).min == 0)
            })
        },
    )
}

fn control_after_skips(suit: Suit, skipped: Vec<Suit>) -> Cons<impl Constraint + Clone> {
    described(
        format!("control in {suit}, denying cheaper available controls"),
        move |hand: Hand, context: &Context<'_>| {
            let controlled = |candidate: Suit| {
                let holding = hand[candidate];
                holding.contains(Rank::A)
                    || holding.contains(Rank::K)
                    || (holding.len() <= 1
                        && context.inferences().partner().length(candidate).min == 0)
            };
            controlled(suit) && skipped.iter().all(|&lower| !controlled(lower))
        },
    )
}

pub(super) fn direct_rkcb(trump: Suit, support: usize) -> Rules {
    Rules::new()
        .rule(
            bid(4, Strain::Notrump),
            500,
            len(trump, support..) & all_side_controls(trump),
        )
        .alert(RKCB)
}

pub(super) fn splinter_and_rkcb(trump: Suit, splinters: &[Suit], support: usize) -> Rules {
    let mut rules = direct_rkcb(trump, support);
    for &short in splinters {
        rules = rules
            .rule(
                bid(4, Strain::from(short)),
                600,
                len(trump, support..) & len(short, 0..=0),
            )
            .alert(Alert("pen:void-splinter"));
    }
    rules
}

fn after_splinter(trump: Suit, splinter: Suit) -> Rules {
    let signoff = bid(4, Strain::from(trump));
    let splinter_bid = bid(4, Strain::from(splinter));
    let mut rules = direct_rkcb(trump, 4);
    let mut skipped = Vec::new();
    for suit in Suit::ASC {
        let cue = bid(4, Strain::from(suit));
        if suit != trump && suit != splinter && cue > splinter_bid && cue < signoff {
            rules = rules
                .rule(
                    cue,
                    400 - i16::try_from(skipped.len()).unwrap_or_default(),
                    control_after_skips(suit, skipped.clone()),
                )
                .alert(CONTROL);
            skipped.push(suit);
        }
    }
    rules.rule(signoff, 100, hcp(0..))
}

fn useful_void(suit: Suit) -> Cons<impl Constraint + Clone> {
    len(suit, 0..=0)
        & described(
            format!("a useful void in {suit}"),
            move |_: Hand, context: &Context<'_>| {
                context.inferences().partner().length(suit).min == 0
            },
        )
}

fn rkcb_answers(trump: Suit) -> Rules {
    let mut rules = Rules::new();
    let lower_voids: Vec<Suit> = Suit::ASC
        .into_iter()
        .filter(|&suit| suit != trump && Strain::from(suit) < Strain::from(trump))
        .collect();
    for (index, suit) in lower_voids.into_iter().enumerate() {
        rules = rules
            .rule(
                bid(6, Strain::from(suit)),
                650 - i16::try_from(index).unwrap_or_default(),
                keycards(trump, &[1, 3]) & useful_void(suit),
            )
            .alert(VOID_RESPONSE);
    }
    let higher_voids: Vec<Suit> = Suit::ASC
        .into_iter()
        .filter(|&suit| suit != trump && Strain::from(suit) > Strain::from(trump))
        .collect();
    if !higher_voids.is_empty() {
        let voids = described(
            "an odd-keycard useful void above the trump suit",
            move |hand: Hand, context: &Context<'_>| {
                higher_voids.iter().any(|&suit| {
                    hand[suit].is_empty() && context.inferences().partner().length(suit).min == 0
                })
            },
        );
        rules = rules
            .rule(
                bid(6, Strain::from(trump)),
                640,
                keycards(trump, &[1, 3]) & voids,
            )
            .alert(VOID_RESPONSE);
    }
    let any_void = described(
        "an even-keycard useful void outside trumps",
        move |hand: Hand, context: &Context<'_>| {
            Suit::ASC.into_iter().any(|suit| {
                suit != trump
                    && hand[suit].is_empty()
                    && context.inferences().partner().length(suit).min == 0
            })
        },
    );
    rules
        .rule(
            bid(5, Strain::Notrump),
            630,
            keycards(trump, &[0, 2, 4]) & any_void,
        )
        .alert(VOID_RESPONSE)
        .rule(bid(5, Strain::Clubs), 300, keycards(trump, &[1, 4]))
        .alert(RKCB)
        .rule(bid(5, Strain::Diamonds), 300, keycards(trump, &[0, 3]))
        .alert(RKCB)
        .rule(
            bid(5, Strain::Hearts),
            300,
            keycards(trump, &[2, 5]) & !trump_queen(trump),
        )
        .alert(RKCB)
        .rule(
            bid(5, Strain::Spades),
            300,
            keycards(trump, &[2, 5]) & trump_queen(trump),
        )
        .alert(RKCB)
}

fn next_bids(after: Bid, count: usize, exclude: Option<Suit>) -> Vec<Bid> {
    let mut bids = Vec::new();
    for level in after.level.get()..=7 {
        for strain in Strain::ASC {
            let candidate = bid(level, strain);
            if candidate > after && candidate.strain.suit() != exclude {
                bids.push(candidate);
                if bids.len() == count {
                    return bids;
                }
            }
        }
    }
    bids
}

fn asker_after_low_answer(trump: Suit, answer: Bid) -> (Rules, Bid) {
    let queen_ask = next_bids(answer, 1, Some(trump))[0];
    let signoff = bid(
        if answer < bid(5, Strain::from(trump)) {
            5
        } else {
            6
        },
        Strain::from(trump),
    );
    (
        Rules::new()
            .rule(queen_ask, 300, hcp(18..))
            .alert(QUEEN_ASK)
            .rule(signoff, 100, hcp(..18)),
        queen_ask,
    )
}

fn queen_answers(trump: Suit, ask: Bid) -> Rules {
    let steps = next_bids(ask, 2, None);
    Rules::new()
        .rule(steps[0], 200, !trump_queen(trump))
        .alert(QUEEN_ASK)
        .rule(steps[1], 200, trump_queen(trump))
        .alert(QUEEN_ASK)
}

fn king_ask() -> Rules {
    Rules::new()
        .rule(bid(5, Strain::Notrump), 200, hcp(18..))
        .alert(KING_ASK)
}

fn specific_king_answers(trump: Suit) -> Rules {
    let mut rules = Rules::new();
    let showable: Vec<Suit> = Suit::ASC
        .into_iter()
        .filter(|&suit| suit != trump && Strain::from(suit) < Strain::from(trump))
        .collect();
    for (index, &suit) in showable.iter().enumerate() {
        let lower = showable[..index].to_vec();
        let constraint = described(
            format!("the lowest safely showable side king is in {suit}"),
            move |hand: Hand, _: &Context<'_>| {
                hand[suit].contains(Rank::K)
                    && lower.iter().all(|&lower| !hand[lower].contains(Rank::K))
            },
        );
        rules = rules
            .rule(bid(6, Strain::from(suit)), 300, constraint)
            .alert(KING_ASK);
    }
    rules.rule(bid(6, Strain::from(trump)), 100, hcp(0..))
}

fn interference_rows(prefix: &str, trump: Suit) -> Vec<Entry> {
    let mut entries = rows_of(
        Pattern::node(&format!("{prefix} (X)")),
        Rules::new()
            .rule(Call::Redouble, 300, keycards(trump, &[0, 3]))
            .alert(ROPI)
            .rule(Call::Pass, 300, keycards(trump, &[1, 4]))
            .alert(ROPI)
            .rule(
                bid(5, Strain::Clubs),
                300,
                keycards(trump, &[2, 5]) & !trump_queen(trump),
            )
            .alert(ROPI)
            .rule(
                bid(5, Strain::Diamonds),
                300,
                keycards(trump, &[2, 5]) & trump_queen(trump),
            )
            .alert(ROPI),
    );
    for strain in Strain::ASC {
        let overcall = bid(5, strain);
        let steps = next_bids(overcall, 2, None);
        entries.extend(rows_of(
            Pattern::node(&format!("{prefix} ({overcall})")),
            Rules::new()
                .rule(Call::Double, 300, keycards(trump, &[0, 3]))
                .alert(DOPI)
                .rule(Call::Pass, 300, keycards(trump, &[1, 4]))
                .alert(DOPI)
                .rule(
                    steps[0],
                    300,
                    keycards(trump, &[2, 5]) & !trump_queen(trump),
                )
                .alert(DOPI)
                .rule(steps[1], 300, keycards(trump, &[2, 5]) & trump_queen(trump))
                .alert(DOPI),
        ));
    }
    for strain in Strain::ASC {
        let overcall = bid(6, strain);
        entries.extend(rows_of(
            Pattern::node(&format!("{prefix} ({overcall})")),
            Rules::new()
                .rule(Call::Double, 300, keycards(trump, &[0, 2, 4]))
                .alert(DEPO)
                .rule(Call::Pass, 300, keycards(trump, &[1, 3, 5]))
                .alert(DEPO),
        ));
    }
    entries
}

fn rkcb_rows(prefix: &str, trump: Suit) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node(prefix), rkcb_answers(trump));
    entries.extend(interference_rows(prefix.trim_end_matches(" -"), trump));
    for answer in [bid(5, Strain::Clubs), bid(5, Strain::Diamonds)] {
        let node = format!("{prefix} {answer} -");
        let (rules, queen_ask) = asker_after_low_answer(trump, answer);
        entries.extend(rows_of(Pattern::node(&node), rules));
        entries.extend(rows_of(
            Pattern::node(&format!("{node} {queen_ask} -")),
            queen_answers(trump, queen_ask),
        ));
    }
    for answer in [bid(5, Strain::Hearts), bid(5, Strain::Spades)] {
        let node = format!("{prefix} {answer} -");
        entries.extend(rows_of(Pattern::node(&node), king_ask()));
        entries.extend(rows_of(
            Pattern::node(&format!("{node} 5NT -")),
            specific_king_answers(trump),
        ));
    }
    entries
}

fn exclusion_asks(trump: Suit) -> Rules {
    let mut rules = direct_rkcb(trump, 0);
    for suit in Suit::ASC {
        if suit != trump {
            rules = rules
                .rule(
                    bid(5, Strain::from(suit)),
                    400,
                    len(suit, 0..=0) & all_side_controls(trump),
                )
                .alert(EXCLUSION);
        }
    }
    rules
}

fn exclusion_rows(base: &str, trump: Suit) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node(base), exclusion_asks(trump));
    for excluded in Suit::ASC {
        if excluded == trump {
            continue;
        }
        let ask = bid(5, Strain::from(excluded));
        let steps = next_bids(ask, 4, None);
        let node = format!("{base} {ask} -");
        entries.extend(rows_of(
            Pattern::node(&node),
            Rules::new()
                .rule(steps[0], 300, exclusion_keycards(trump, excluded, &[1, 4]))
                .alert(EXCLUSION)
                .rule(steps[1], 300, exclusion_keycards(trump, excluded, &[0, 3]))
                .alert(EXCLUSION)
                .rule(
                    steps[2],
                    300,
                    exclusion_keycards(trump, excluded, &[2]) & !trump_queen(trump),
                )
                .alert(EXCLUSION)
                .rule(
                    steps[3],
                    300,
                    exclusion_keycards(trump, excluded, &[2]) & trump_queen(trump),
                )
                .alert(EXCLUSION),
        ));
    }
    entries
}

fn entries(_: &Agreements) -> Vec<Entry> {
    let anchors = [
        ("P* 1♦ - 4NT -", Suit::Spades),
        ("P* 1♥ - 4NT -", Suit::Hearts),
        ("P* 1♦ - 1♥ - 1NT - 4NT -", Suit::Hearts),
        ("P* 1♦ - 1♠ - 1NT - 4NT -", Suit::Spades),
        ("P* 1♠ - 2♦ - 2NT - 3♥ - 4NT -", Suit::Hearts),
        ("P* 1♠ - 2♥ - 2NT - 3♠ - 4NT -", Suit::Spades),
        ("P* 1NT - 4♦ - 4♥ - 4NT -", Suit::Hearts),
        ("P* 1NT - 4♥ - 4♠ - 4NT -", Suit::Spades),
        ("P* 1♣ - 1♥ - 2♥ - 4NT -", Suit::Hearts),
        ("P* 1♣ - 1♠ - 2♠ - 4NT -", Suit::Spades),
        ("P* 1♣ - 2♣ - 3♣ - 4NT -", Suit::Clubs),
        ("P* 1♣ - 2♦ - 3♦ - 4NT -", Suit::Diamonds),
        ("P* 1♦ - 1♠ - 4NT -", Suit::Spades),
        ("P* 1♦ - 2♠ - 4NT -", Suit::Spades),
        ("P* 1♥ - 2♥ - 4NT -", Suit::Hearts),
        ("P* 1♥ - 2NT - 4NT -", Suit::Hearts),
        ("P* 1♥ - 3♥ - 4NT -", Suit::Hearts),
        ("P* 2♦ - 2NT - 4NT -", Suit::Diamonds),
        ("P* 1♦ - 4♣ - 4NT -", Suit::Spades),
        ("P* 1♦ - 4♦ - 4NT -", Suit::Spades),
        ("P* 1♦ - 4♥ - 4NT -", Suit::Spades),
        ("P* 1♥ - 4♣ - 4NT -", Suit::Hearts),
        ("P* 1♥ - 4♦ - 4NT -", Suit::Hearts),
    ];
    let mut entries = Vec::new();
    for (prefix, trump) in anchors {
        entries.extend(rkcb_rows(prefix, trump));
    }

    for (base, trump, splinter) in [
        ("P* 1♦ - 4♣ -", Suit::Spades, Suit::Clubs),
        ("P* 1♦ - 4♦ -", Suit::Spades, Suit::Diamonds),
        ("P* 1♦ - 4♥ -", Suit::Spades, Suit::Hearts),
        ("P* 1♥ - 4♣ -", Suit::Hearts, Suit::Clubs),
        ("P* 1♥ - 4♦ -", Suit::Hearts, Suit::Diamonds),
    ] {
        entries.extend(rows_of(
            Pattern::node(base),
            after_splinter(trump, splinter),
        ));
    }

    for (base, trump) in [
        ("P* 1♣ - 1♥ - 2♥ -", Suit::Hearts),
        ("P* 1♣ - 1♠ - 2♠ -", Suit::Spades),
        ("P* 1♣ - 2♣ - 3♣ -", Suit::Clubs),
        ("P* 1♣ - 2♦ - 3♦ -", Suit::Diamonds),
        ("P* 1♦ - 1♠ -", Suit::Spades),
        ("P* 1♦ - 2♠ -", Suit::Spades),
        ("P* 1♦ - 1♥ - 1NT -", Suit::Hearts),
        ("P* 1♦ - 1♠ - 1NT -", Suit::Spades),
        ("P* 1♥ - 2♥ -", Suit::Hearts),
        ("P* 1♥ - 2NT -", Suit::Hearts),
        ("P* 1♥ - 3♥ -", Suit::Hearts),
        ("P* 1♠ - 2♦ - 2NT - 3♥ -", Suit::Hearts),
        ("P* 1♠ - 2♥ - 2NT - 3♠ -", Suit::Spades),
        ("P* 2♦ - 2NT -", Suit::Diamonds),
    ] {
        entries.extend(exclusion_rows(base, trump));
    }
    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-slam",
        gate: |_| true,
        entries,
    }
}
