//! PEN-Club opening constraints.

use crate::bidding::constraint::{Cons, Constraint, balanced, hcp, len, longer_suit, reads_as};
use crate::bidding::rows::{Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::{Bid, Strain, Suit};

use super::strength::preempt_strength;

pub(super) const STRONG_CLUB: Alert = Alert("pen:strong-club");
pub(super) const SPADE_OPENING: Alert = Alert("pen:one-spade-union");
pub(super) const SPADE_DIAMOND: Alert = Alert("pen:spade-diamond-opening");
pub(super) const WEAK_NOTRUMP: Alert = Alert("pen:weak-notrump");

pub(super) fn one_notrump_shape() -> Cons<impl Constraint + Clone> {
    (balanced()
        | (len(Suit::Clubs, 1..=1)
            & len(Suit::Diamonds, 4..=4)
            & len(Suit::Hearts, 4..=4)
            & len(Suit::Spades, 4..=4))
        | (len(Suit::Diamonds, 1..=1)
            & len(Suit::Clubs, 4..=4)
            & len(Suit::Hearts, 4..=4)
            & len(Suit::Spades, 4..=4)))
        & len(Suit::Hearts, 3..)
        & len(Suit::Spades, 3..)
}

pub(super) fn one_notrump() -> Cons<impl Constraint + Clone> {
    hcp(12..=15) & one_notrump_shape()
}

pub(super) fn strong_one_spade() -> Cons<impl Constraint + Clone> {
    hcp(16..=19)
        & !balanced()
        & len(Suit::Clubs, 5..)
        & longer_suit(Suit::Clubs, Suit::Diamonds)
        & longer_suit(Suit::Clubs, Suit::Hearts)
        & longer_suit(Suit::Clubs, Suit::Spades)
}

pub(super) fn limited_one_spade() -> Cons<impl Constraint + Clone> {
    hcp(10..=15)
        & len(Suit::Clubs, 4..)
        & len(Suit::Diamonds, 4..)
        & len(Suit::Hearts, ..4)
        & len(Suit::Spades, ..4)
        & !(len(Suit::Clubs, 6..) & len(Suit::Diamonds, 4..=4))
        & !(len(Suit::Diamonds, 6..) & len(Suit::Clubs, 4..=4))
}

pub(super) fn rules() -> Rules {
    let strong_spade = strong_one_spade();
    let five_two_three_three = len(Suit::Spades, 5..=5)
        & len(Suit::Hearts, 2..=2)
        & len(Suit::Diamonds, 3..=3)
        & len(Suit::Clubs, 3..=3);
    let one_diamond = hcp(10..=15)
        & (!balanced() | five_two_three_three)
        & len(Suit::Spades, 4..)
        & !one_notrump();
    let one_heart = hcp(10..=15)
        & !balanced()
        & len(Suit::Hearts, 4..)
        & len(Suit::Spades, ..4)
        & !one_notrump();
    let one_spade = limited_one_spade() | strong_spade.clone();
    let one_club = hcp(16..) & !strong_spade & !(hcp(22..=24) & balanced());
    Rules::new()
        .rule(Bid::new(1, Strain::Notrump), 300, one_notrump())
        .alert(WEAK_NOTRUMP)
        .rule(
            Bid::new(1, Strain::Diamonds),
            250,
            reads_as(one_diamond, hcp(10..=15) & len(Suit::Spades, 4..)),
        )
        .alert(SPADE_DIAMOND)
        .rule(
            Bid::new(1, Strain::Hearts),
            240,
            reads_as(
                one_heart,
                hcp(10..=15) & len(Suit::Hearts, 4..) & len(Suit::Spades, ..4),
            ),
        )
        .rule(
            Bid::new(2, Strain::Clubs),
            230,
            hcp(11..=15)
                & len(Suit::Clubs, 6..)
                & len(Suit::Diamonds, ..=4)
                & len(Suit::Hearts, ..4)
                & len(Suit::Spades, ..4),
        )
        .rule(
            Bid::new(2, Strain::Diamonds),
            230,
            hcp(11..=15)
                & len(Suit::Diamonds, 6..)
                & len(Suit::Clubs, ..=4)
                & len(Suit::Hearts, ..4)
                & len(Suit::Spades, ..4),
        )
        .rule(
            Bid::new(1, Strain::Spades),
            220,
            reads_as(
                one_spade,
                (hcp(10..=15)
                    & len(Suit::Clubs, 4..)
                    & len(Suit::Diamonds, 4..)
                    & len(Suit::Hearts, ..4)
                    & len(Suit::Spades, ..4))
                    | (hcp(16..=19) & len(Suit::Clubs, 5..)),
            ),
        )
        .alert(SPADE_OPENING)
        .rule(
            Bid::new(1, Strain::Clubs),
            210,
            reads_as(one_club, hcp(16..)),
        )
        .alert(STRONG_CLUB)
        .rule(
            Bid::new(2, Strain::Hearts),
            200,
            preempt_strength(Suit::Hearts) & len(Suit::Hearts, 6..) & len(Suit::Spades, ..6),
        )
        .rule(
            Bid::new(2, Strain::Spades),
            200,
            preempt_strength(Suit::Spades) & len(Suit::Spades, 6..) & len(Suit::Hearts, ..6),
        )
        .rule(Bid::new(2, Strain::Notrump), 190, hcp(22..=24) & balanced())
        .rule(
            Bid::new(3, Strain::Clubs),
            210,
            preempt_strength(Suit::Clubs) & len(Suit::Clubs, 7..),
        )
        .rule(
            Bid::new(3, Strain::Diamonds),
            210,
            preempt_strength(Suit::Diamonds) & len(Suit::Diamonds, 7..),
        )
        .rule(
            Bid::new(3, Strain::Hearts),
            210,
            preempt_strength(Suit::Hearts) & len(Suit::Hearts, 7..),
        )
        .rule(
            Bid::new(3, Strain::Spades),
            210,
            preempt_strength(Suit::Spades) & len(Suit::Spades, 7..),
        )
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-openings",
        gate: |_| true,
        entries: |_| rows_of(Pattern::node("P*"), rules()),
    }
}
