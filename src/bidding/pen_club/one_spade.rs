//! The artificial two-branch 1♠ opening and its constructive continuations.

use super::openings::{limited_one_spade, strong_one_spade};
use super::strength::limited_maximum;
use crate::bidding::agreements::Agreements;
use crate::bidding::constraint::{Cons, Constraint, at_least_as_long, hcp, len, longer_suit};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Strain, Suit};

pub(super) const PREFERENCE: Alert = Alert("pen:one-spade-preference");
pub(super) const MAJOR_TRANSFER: Alert = Alert("pen:major-transfer");
pub(super) const TRANSFER_COMPLETION: Alert = Alert("pen:transfer-completion");
const GENERAL_ASK: Alert = Alert("pen:one-spade-general-ask");
const SHAPE_ASK: Alert = Alert("pen:one-spade-shape-ask");
const BRANCH_ANSWER: Alert = Alert("pen:one-spade-branch-answer");
const SUPPORT_REBID: Alert = Alert("pen:one-spade-support-rebid");
const SHAPE_RELAY: Alert = Alert("pen:one-spade-shape-relay");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

fn strong_hearts() -> Cons<impl Constraint + Clone> {
    strong_one_spade() & len(Suit::Hearts, 4..)
}

fn strong_spades() -> Cons<impl Constraint + Clone> {
    strong_one_spade() & len(Suit::Hearts, ..4) & len(Suit::Spades, 4..)
}

fn strong_diamonds() -> Cons<impl Constraint + Clone> {
    strong_one_spade() & len(Suit::Hearts, ..4) & len(Suit::Spades, ..4) & len(Suit::Diamonds, 4..)
}

fn strong_clubs_only() -> Cons<impl Constraint + Clone> {
    strong_one_spade() & len(Suit::Hearts, ..4) & len(Suit::Spades, ..4) & len(Suit::Diamonds, ..4)
}

/// Five-card majors take their transfer even with strength.  Otherwise 12 is
/// the no-fit INV floor and 15 the no-fit GF floor opposite a ten-point branch.
pub(super) fn responses() -> Rules {
    Rules::new()
        .rule(
            bid(2, Strain::Diamonds),
            500,
            len(Suit::Hearts, 5..) & at_least_as_long(Suit::Hearts, Suit::Spades),
        )
        .alert(MAJOR_TRANSFER)
        .rule(
            bid(2, Strain::Hearts),
            500,
            len(Suit::Spades, 5..) & longer_suit(Suit::Spades, Suit::Hearts),
        )
        .alert(MAJOR_TRANSFER)
        .rule(
            bid(2, Strain::Notrump),
            400,
            hcp(15..) & len(Suit::Hearts, ..5) & len(Suit::Spades, ..5),
        )
        .alert(SHAPE_ASK)
        .rule(
            bid(2, Strain::Spades),
            380,
            hcp(12..) & len(Suit::Hearts, ..5) & len(Suit::Spades, ..5),
        )
        .alert(GENERAL_ASK)
        .rule(
            bid(1, Strain::Notrump),
            100,
            at_least_as_long(Suit::Diamonds, Suit::Clubs),
        )
        .alert(PREFERENCE)
        .rule(
            bid(2, Strain::Clubs),
            100,
            longer_suit(Suit::Clubs, Suit::Diamonds),
        )
        .alert(PREFERENCE)
}

pub(super) fn transfer_only_responses() -> Rules {
    Rules::new()
        .rule(
            bid(2, Strain::Diamonds),
            500,
            len(Suit::Hearts, 5..) & at_least_as_long(Suit::Hearts, Suit::Spades),
        )
        .alert(MAJOR_TRANSFER)
        .rule(
            bid(2, Strain::Hearts),
            500,
            len(Suit::Spades, 5..) & longer_suit(Suit::Spades, Suit::Hearts),
        )
        .alert(MAJOR_TRANSFER)
        .rule(
            bid(2, Strain::Notrump),
            400,
            hcp(15..) & len(Suit::Hearts, ..5) & len(Suit::Spades, ..5),
        )
        .alert(SHAPE_ASK)
        .rule(
            bid(2, Strain::Spades),
            380,
            hcp(12..) & len(Suit::Hearts, ..5) & len(Suit::Spades, ..5),
        )
        .alert(GENERAL_ASK)
}

pub(super) fn responses_after_double() -> Rules {
    responses()
        .rule(
            Call::Redouble,
            390,
            hcp(12..) & len(Suit::Hearts, ..5) & len(Suit::Spades, ..5),
        )
        .alert(GENERAL_ASK)
}

fn preference_diamond_rebids() -> Rules {
    Rules::new()
        .rule(
            bid(3, Strain::Diamonds),
            500,
            limited_one_spade() & limited_maximum() & len(Suit::Diamonds, 5..),
        )
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Hearts), 450, strong_hearts())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Spades), 440, strong_spades())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Notrump), 430, strong_diamonds())
        .alert(BRANCH_ANSWER)
        .rule(bid(3, Strain::Clubs), 420, strong_clubs_only())
        .alert(BRANCH_ANSWER)
        .rule(
            Call::Pass,
            300,
            limited_one_spade() & len(Suit::Clubs, ..=4) & len(Suit::Diamonds, ..=4),
        )
        .rule(
            bid(2, Strain::Clubs),
            220,
            limited_one_spade() & longer_suit(Suit::Clubs, Suit::Diamonds),
        )
        .rule(
            bid(2, Strain::Diamonds),
            200,
            limited_one_spade() & at_least_as_long(Suit::Diamonds, Suit::Clubs),
        )
}

fn preference_club_rebids() -> Rules {
    Rules::new()
        .rule(
            bid(3, Strain::Clubs),
            500,
            limited_one_spade() & limited_maximum() & len(Suit::Clubs, 5..),
        )
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Hearts), 450, strong_hearts())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Spades), 440, strong_spades())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Diamonds), 430, strong_diamonds())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Notrump), 420, strong_clubs_only())
        .alert(BRANCH_ANSWER)
        .rule(Call::Pass, 200, limited_one_spade())
}

pub(super) fn general_ask_answers() -> Rules {
    Rules::new()
        .rule(bid(3, Strain::Hearts), 500, strong_hearts())
        .alert(BRANCH_ANSWER)
        .rule(bid(3, Strain::Spades), 490, strong_spades())
        .alert(BRANCH_ANSWER)
        .rule(bid(3, Strain::Diamonds), 480, strong_diamonds())
        .alert(BRANCH_ANSWER)
        .rule(bid(3, Strain::Notrump), 470, strong_clubs_only())
        .alert(BRANCH_ANSWER)
        .rule(
            bid(2, Strain::Notrump),
            300,
            limited_one_spade() & at_least_as_long(Suit::Diamonds, Suit::Clubs),
        )
        .alert(BRANCH_ANSWER)
        .rule(
            bid(3, Strain::Clubs),
            300,
            limited_one_spade() & longer_suit(Suit::Clubs, Suit::Diamonds),
        )
        .alert(BRANCH_ANSWER)
}

/// The same branch disclosure after their double, with Redouble occupying the
/// general-ask slot and therefore preserving the lower answer ladder.
pub(super) fn redouble_ask_answers() -> Rules {
    Rules::new()
        .rule(bid(2, Strain::Hearts), 500, strong_hearts())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Spades), 490, strong_spades())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Diamonds), 480, strong_diamonds())
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::Notrump), 470, strong_clubs_only())
        .alert(BRANCH_ANSWER)
        .rule(
            bid(1, Strain::Notrump),
            300,
            limited_one_spade() & at_least_as_long(Suit::Diamonds, Suit::Clubs),
        )
        .alert(BRANCH_ANSWER)
        .rule(
            bid(2, Strain::Clubs),
            300,
            limited_one_spade() & longer_suit(Suit::Clubs, Suit::Diamonds),
        )
        .alert(BRANCH_ANSWER)
}

pub(super) fn responses_over_one_notrump() -> Rules {
    transfer_only_responses()
        .rule(Call::Double, 450, hcp(10..))
        .rule(
            bid(2, Strain::Clubs),
            100,
            longer_suit(Suit::Clubs, Suit::Diamonds),
        )
        .alert(PREFERENCE)
}

pub(super) fn responses_over_two_clubs() -> Rules {
    transfer_only_responses().rule(Call::Double, 450, hcp(8..) & len(Suit::Clubs, 4..))
}

pub(super) fn responses_over_two_diamonds() -> Rules {
    Rules::new()
        .rule(
            Call::Double,
            500,
            len(Suit::Hearts, 5..) & at_least_as_long(Suit::Hearts, Suit::Spades),
        )
        .alert(MAJOR_TRANSFER)
        .rule(
            bid(2, Strain::Hearts),
            500,
            len(Suit::Spades, 5..) & longer_suit(Suit::Spades, Suit::Hearts),
        )
        .alert(MAJOR_TRANSFER)
        .rule(bid(2, Strain::Notrump), 400, hcp(15..))
        .alert(SHAPE_ASK)
        .rule(bid(2, Strain::Spades), 380, hcp(12..))
        .alert(GENERAL_ASK)
}

pub(super) fn responses_over_two_hearts() -> Rules {
    Rules::new()
        .rule(Call::Double, 500, len(Suit::Spades, 5..))
        .alert(MAJOR_TRANSFER)
        .rule(bid(2, Strain::Notrump), 400, hcp(15..))
        .alert(SHAPE_ASK)
        .rule(bid(2, Strain::Spades), 380, hcp(12..))
        .alert(GENERAL_ASK)
}

pub(super) fn shape_ask_answers() -> Rules {
    Rules::new()
        .rule(bid(3, Strain::Hearts), 500, strong_hearts())
        .alert(BRANCH_ANSWER)
        .rule(bid(3, Strain::Spades), 490, strong_spades())
        .alert(BRANCH_ANSWER)
        .rule(
            bid(3, Strain::Notrump),
            480,
            strong_diamonds() | strong_clubs_only(),
        )
        .alert(BRANCH_ANSWER)
        .rule(
            bid(3, Strain::Clubs),
            300,
            limited_one_spade() & longer_suit(Suit::Clubs, Suit::Diamonds),
        )
        .alert(BRANCH_ANSWER)
        .rule(
            bid(3, Strain::Diamonds),
            300,
            limited_one_spade() & at_least_as_long(Suit::Diamonds, Suit::Clubs),
        )
        .alert(BRANCH_ANSWER)
}

fn detailed_shape_relay() -> Rules {
    Rules::new()
        .rule(bid(4, Strain::Clubs), 100, hcp(17..))
        .alert(SHAPE_RELAY)
}

fn detailed_shape_relay_answers() -> Rules {
    Rules::new()
        .rule(bid(4, Strain::Diamonds), 200, strong_diamonds())
        .alert(BRANCH_ANSWER)
        .rule(bid(4, Strain::Hearts), 200, strong_clubs_only())
        .alert(BRANCH_ANSWER)
}

fn transfer_rebids(target: Suit) -> Rules {
    let other_major = if target == Suit::Hearts {
        Suit::Spades
    } else {
        Suit::Hearts
    };
    let mut rules = Rules::new()
        .rule(
            bid(3, Strain::from(target)),
            600,
            limited_one_spade() & limited_maximum() & len(target, 3..=3),
        )
        .alert(TRANSFER_COMPLETION)
        .rule(
            bid(2, Strain::Notrump),
            550,
            strong_one_spade() & len(target, 3..),
        )
        .alert(SUPPORT_REBID);

    if target == Suit::Hearts {
        rules = rules.rule(
            bid(2, Strain::Spades),
            500,
            strong_one_spade() & len(target, ..3) & len(other_major, 4..),
        );
    } else {
        rules = rules.rule(
            bid(3, Strain::Hearts),
            500,
            strong_one_spade() & len(target, ..3) & len(other_major, 4..),
        );
    }
    rules
        .alert(BRANCH_ANSWER)
        .rule(
            bid(3, Strain::Diamonds),
            480,
            strong_one_spade()
                & len(target, ..3)
                & len(other_major, ..4)
                & len(Suit::Diamonds, 4..),
        )
        .alert(BRANCH_ANSWER)
        .rule(
            bid(3, Strain::Clubs),
            470,
            strong_one_spade()
                & len(target, ..3)
                & len(other_major, ..4)
                & len(Suit::Diamonds, ..4),
        )
        .alert(BRANCH_ANSWER)
        .rule(bid(2, Strain::from(target)), 100, limited_one_spade())
        .alert(TRANSFER_COMPLETION)
}

pub(super) fn complete_transfer(target: Suit) -> Rules {
    transfer_rebids(target)
}

pub(super) fn forced_transfer_signoff(target: Suit) -> Rules {
    Rules::new()
        .rule(bid(3, Strain::from(target)), 100, hcp(0..))
        .alert(TRANSFER_COMPLETION)
}

fn club_signoff() -> Rules {
    Rules::new()
        .rule(bid(3, Strain::Clubs), 100, hcp(..=11))
        .alert(TRANSFER_COMPLETION)
}

fn entries(_: &Agreements) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node("P* 1♠ -"), responses());
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 1NT -"),
        preference_diamond_rebids(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2♣ -"),
        preference_club_rebids(),
    ));
    entries.extend(rows_of(Pattern::node("P* 1♠ - 2♣ - 2NT -"), club_signoff()));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2♠ -"),
        general_ask_answers(),
    ));
    entries.extend(rows_of(Pattern::node("P* 1♠ - 2NT -"), shape_ask_answers()));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2NT - 3NT -"),
        detailed_shape_relay(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2NT - 3NT - 4♣ -"),
        detailed_shape_relay_answers(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2♦ -"),
        transfer_rebids(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2♦ - 2NT -"),
        forced_transfer_signoff(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2♥ -"),
        transfer_rebids(Suit::Spades),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ - 2♥ - 2NT -"),
        forced_transfer_signoff(Suit::Spades),
    ));
    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-one-spade",
        gate: |_| true,
        entries,
    }
}
