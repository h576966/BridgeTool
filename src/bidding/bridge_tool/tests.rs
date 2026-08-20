use super::*;

fn hand(text: &str) -> Hand {
    let hand: Hand = text.parse().expect("valid test hand");
    assert_eq!(hand.len(), 13, "test holding must contain 13 cards: {text}");
    hand
}

fn is_eligible(text: &str, opening: Opening) -> bool {
    eligible_openings(hand(text)).contains(&opening)
}

fn assert_hcp(text: &str, expected: u8) {
    assert_eq!(HandFacts::from(hand(text)).hcp, expected, "{text}");
}

#[test]
fn hand_facts_expose_only_objective_information() {
    let facts = HandFacts::from(hand("AKQ2.J43.432.432"));

    assert_eq!(facts.hcp, 10);
    assert_eq!(facts.suit_lengths, [3, 3, 3, 4]);
    assert_eq!(facts.suit_hcp, [0, 0, 1, 9]);
    assert_eq!(facts.shape, [4, 3, 3, 3]);
    assert!(facts.singletons.is_empty());
    assert!(facts.voids.is_empty());

    let singleton = HandFacts::from(hand("32.3.5432.AKQJ87"));
    assert_eq!(singleton.singletons, vec![Suit::Hearts]);
    assert!(singleton.voids.is_empty());

    let void = HandFacts::from(hand("KQJ98.AT76.5432."));
    assert!(void.singletons.is_empty());
    assert_eq!(void.voids, vec![Suit::Clubs]);
}

#[test]
fn unbalanced_definition_is_shape_based_and_suit_independent() {
    for balanced in ["AKQ2.J43.432.432", "AKQJ2.43.432.432"] {
        assert!(
            !HandFacts::from(hand(balanced)).is_unbalanced(),
            "{balanced}"
        );
    }

    for unbalanced in [
        "AKQJ2.9876.54.32",
        "AKQ2.J432.9876.5",
        "5.AKQ2.J432.9876",
        "AKQJ98.543.32.42",
    ] {
        assert!(
            HandFacts::from(hand(unbalanced)).is_unbalanced(),
            "{unbalanced}"
        );
    }
}

#[test]
fn one_notrump_accepts_every_permitted_major_length_pair() {
    for text in [
        "AK2.QJ2.A432.432",
        "AK32.QJ2.A32.432",
        "AK2.QJ32.A32.432",
        "AK32.QJ32.A32.32",
        "AK432.QJ2.A32.32",
        "AK2.QJ432.A32.32",
    ] {
        assert!(is_eligible(text, Opening::OneNotrump), "{text}");
    }
}

#[test]
fn one_notrump_accepts_only_the_stated_shapes() {
    for allowed in [
        "AK32.QJ2.A32.432",
        "AK32.QJ32.A32.32",
        "AK432.QJ2.A32.32",
        "AK32.QJ43.A432.2",
        "AK32.QJ43.2.A432",
    ] {
        assert!(is_eligible(allowed, Opening::OneNotrump), "{allowed}");
    }

    for rejected in [
        "AK.QJ2.A5432.432",
        "AK32.QJ.A432.432",
        "AK432.QJ43.A2.32",
        "AKQ432.J32.A2.32",
        "AK32.2.A432.QJ43",
        "2.AK32.A432.QJ43",
    ] {
        assert!(!is_eligible(rejected, Opening::OneNotrump), "{rejected}");
    }
}

#[test]
fn minor_singleton_4441_is_selected_as_one_notrump() {
    let hand = hand("AQ32.KJ54.Q432.2");

    assert_eq!(
        eligible_openings(hand),
        vec![Opening::OneDiamond, Opening::OneNotrump]
    );
    assert_eq!(
        select_opening(hand),
        OpeningSelection::Selected(Opening::OneNotrump)
    );
}

#[test]
fn both_one_spade_variants_are_eligible() {
    let minor_two_suiter = "432.32.AKQJ.AJ87";
    let strong_clubs = "32.Q3.AK2.AKQJ87";

    assert_hcp(minor_two_suiter, 15);
    assert!(is_eligible(minor_two_suiter, Opening::OneSpade));
    assert_hcp(strong_clubs, 19);
    assert!(is_eligible(strong_clubs, Opening::OneSpade));
}

#[test]
fn minor_two_suiter_one_spade_denies_a_four_card_major() {
    assert!(!is_eligible("AKQ2.3.KQJ2.8765", Opening::OneSpade));
    assert!(!is_eligible("3.AKQ2.KQJ2.8765", Opening::OneSpade));
}

#[test]
fn one_club_excludes_only_the_qualifying_strong_club_variant() {
    let qualifying = "32.Q3.AK2.AKQJ87";
    assert_eq!(eligible_openings(hand(qualifying)), vec![Opening::OneSpade]);

    let equal_length = "2.A3.KQ432.AKJ87";
    assert_hcp(equal_length, 17);
    assert_eq!(
        eligible_openings(hand(equal_length)),
        vec![Opening::OneClub]
    );

    let over_range = "32.K3.AK2.AKQJ87";
    assert_hcp(over_range, 20);
    assert_eq!(eligible_openings(hand(over_range)), vec![Opening::OneClub]);
}

#[test]
fn major_openings_allow_a_longer_side_suit() {
    assert!(is_eligible("AKQ2.J8765.3.432", Opening::OneDiamond));
    assert!(is_eligible("32.AKQ2.3.J87654", Opening::OneHeart));
}

#[test]
fn long_minor_openings_allow_exactly_four_of_the_other_minor() {
    assert!(is_eligible("A2.3.5432.AKQJ87", Opening::TwoClubs));
    assert!(is_eligible("A2.3.AKQJ87.5432", Opening::TwoDiamonds));

    assert!(!is_eligible("A.3.T5432.AKQJ87", Opening::TwoClubs));
    assert!(!is_eligible("A.3.AKQJ87.T5432", Opening::TwoDiamonds));
    assert!(!is_eligible("A432.3.32.AKQJ87", Opening::TwoClubs));
    assert!(!is_eligible("A432.3.AKQJ87.32", Opening::TwoDiamonds));
}

#[test]
fn six_four_minor_assignment_is_visible_in_the_diagnostic() {
    let clubs = hand("A2.3.5432.AKQJ87");
    let diamonds = hand("A2.3.AKQJ87.5432");

    assert_eq!(minor_exception_candidates(clubs), vec![Opening::TwoClubs]);
    assert_eq!(
        minor_exception_candidates(diamonds),
        vec![Opening::TwoDiamonds]
    );
    assert!(eligible_openings(clubs).contains(&Opening::TwoClubs));
    assert!(eligible_openings(diamonds).contains(&Opening::TwoDiamonds));

    let wrong_side_suit = hand("32.5432.3.AKQJ87");
    assert!(minor_exception_candidates(wrong_side_suit).is_empty());
}

#[test]
fn hcp_boundaries_cover_every_limited_opening() {
    let cases = [
        (
            Opening::OneDiamond,
            "AKQ2.3.432.87654",
            "AKQ2.3.432.J8765",
            "AKQJ.3.A32.J8765",
            "AKQJ.3.K32.K8765",
            [9, 10, 15, 16],
        ),
        (
            Opening::OneHeart,
            "32.AKQ2.43.87654",
            "32.AKQ2.43.J8765",
            "32.AKQJ.A3.J8765",
            "32.AKQJ.K3.K8765",
            [9, 10, 15, 16],
        ),
        (
            Opening::OneSpade,
            "432.32.AKQ2.8765",
            "432.32.AKQ2.J876",
            "432.32.AKQJ.AJ87",
            "432.32.AKQJ.KQJ8",
            [9, 10, 15, 16],
        ),
        (
            Opening::OneNotrump,
            "AQ32.K32.J42.J32",
            "AQ32.KJ2.J42.J32",
            "AKQJ.K32.Q42.432",
            "AKQJ.K32.Q42.J32",
            [11, 12, 15, 16],
        ),
        (
            Opening::TwoClubs,
            "432.432.2.AKQJ87",
            "J32.432.2.AKQJ87",
            "A3.432.J2.AKQJ87",
            "Q3.K32.J2.AKQJ87",
            [10, 11, 15, 16],
        ),
        (
            Opening::TwoDiamonds,
            "432.432.AKQJ87.2",
            "J32.432.AKQJ87.2",
            "A3.432.AKQJ87.J2",
            "Q3.K32.AKQJ87.J2",
            [10, 11, 15, 16],
        ),
    ];

    for (opening, below, low, high, above, expected_hcp) in cases {
        for (text, expected_hcp) in [below, low, high, above].into_iter().zip(expected_hcp) {
            assert_hcp(text, expected_hcp);
        }
        assert!(!is_eligible(below, opening), "{opening} below");
        assert!(is_eligible(low, opening), "{opening} low");
        assert!(is_eligible(high, opening), "{opening} high");
        assert!(!is_eligible(above, opening), "{opening} above");
    }
}

#[test]
fn one_club_and_strong_one_spade_have_their_stated_hcp_boundaries() {
    let club_15 = "AKQJ.K32.Q42.432";
    let club_16 = "AKQJ.K32.Q42.J32";
    assert_hcp(club_15, 15);
    assert_hcp(club_16, 16);
    assert!(!is_eligible(club_15, Opening::OneClub));
    assert!(is_eligible(club_16, Opening::OneClub));

    for (text, hcp, expected) in [
        ("32.Q3.K32.AKQJ87", 15, false),
        ("32.Q3.A32.AKQJ87", 16, true),
        ("32.Q3.AK2.AKQJ87", 19, true),
        ("32.K3.AK2.AKQJ87", 20, false),
    ] {
        assert_hcp(text, hcp);
        assert_eq!(is_eligible(text, Opening::OneSpade), expected, "{text}");
    }
}

#[test]
fn no_match_and_four_card_major_routing_remain_explicit() {
    let no_match = hand("AQ32.K32.J42.J32");
    assert_eq!(eligible_openings(no_match), Vec::<Opening>::new());
    assert_eq!(select_opening(no_match), OpeningSelection::NoMatch);

    let four_spades = hand("AKQ2.3.KQJ2.8765");
    assert_eq!(eligible_openings(four_spades), vec![Opening::OneDiamond]);
    assert_eq!(
        select_opening(four_spades),
        OpeningSelection::Selected(Opening::OneDiamond)
    );
}
