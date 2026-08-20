//! Explicit competitive tails for PEN-Club's artificial calls.

use super::constructive::{
    complete_minor_transfer, complete_transfer, one_spade_responses, transfer_only_responses,
};
use crate::bidding::agreements::Agreements;
use crate::bidding::american::NotrumpDefense;
use crate::bidding::constraint::{hcp, len};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Strain, Suit};

const TWO_DIAMOND_TRANSFER: Alert = Alert("pen:two-diamond-doubled-transfer");
const TRANSFER_COMPLETION: Alert = Alert("pen:transfer-completion");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

fn two_diamond_doubled_responses() -> Rules {
    Rules::new()
        .rule(Call::Redouble, 300, len(Suit::Hearts, 5..))
        .alert(TWO_DIAMOND_TRANSFER)
        .rule(bid(2, Strain::Hearts), 300, len(Suit::Spades, 5..))
        .alert(TWO_DIAMOND_TRANSFER)
}

fn forced(call: Bid) -> Rules {
    Rules::new()
        .rule(call, 100, hcp(0..))
        .alert(TRANSFER_COMPLETION)
}

fn natural_major_overcall(over: Suit) -> Rules {
    let other = if over == Suit::Hearts {
        Suit::Spades
    } else {
        Suit::Hearts
    };
    let level = if over == Suit::Hearts { 2 } else { 3 };
    Rules::new()
        .rule(Call::Double, 300, hcp(9..) & len(over, 4..))
        .rule(
            bid(level, Strain::from(other)),
            200,
            hcp(..=8) & len(other, 4..),
        )
}

fn entries(agreements: &Agreements) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node("P* 1♠ (X)"), one_spade_responses());
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♣)"),
        transfer_only_responses(),
    ));

    for pattern in ["P* 1♠ - 2♦ (X)", "P* 1♠ (X) 2♦ -", "P* 1♠ (2♣) 2♦ -"] {
        entries.extend(rows_of(
            Pattern::node(pattern),
            complete_transfer(Suit::Hearts),
        ));
    }
    for pattern in ["P* 1♠ - 2♥ (X)", "P* 1♠ (X) 2♥ -", "P* 1♠ (2♣) 2♥ -"] {
        entries.extend(rows_of(
            Pattern::node(pattern),
            complete_transfer(Suit::Spades),
        ));
    }

    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♦ (X)"),
        complete_transfer(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♥ (X)"),
        complete_transfer(Suit::Spades),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♠ (X)"),
        complete_minor_transfer(Suit::Clubs),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2NT (X)"),
        complete_minor_transfer(Suit::Diamonds),
    ));

    entries.extend(rows_of(
        Pattern::node("P* 2♦ (X)"),
        two_diamond_doubled_responses(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 2♦ (X) XX -"),
        forced(bid(2, Strain::Hearts)),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 2♦ (X) 2♥ -"),
        forced(bid(2, Strain::Spades)),
    ));

    // The new 1♦ guarantees spades. This is the one low-interference response
    // whose old PEN meaning survives unchanged: a natural three-plus-card fit.
    entries.extend(rows_of(
        Pattern::node("P* 1♦ (1♥)"),
        Rules::new().rule(
            bid(1, Strain::Spades),
            200,
            hcp(6..) & len(Suit::Spades, 3..),
        ),
    ));

    // Only a declared natural defense activates these calls. Artificial 2M
    // conventions using the same face must not be treated as penalty targets.
    if agreements.decision.reading.notrump_defense == NotrumpDefense::Natural {
        entries.extend(rows_of(
            Pattern::node("P* 1NT (2♥)"),
            natural_major_overcall(Suit::Hearts),
        ));
        entries.extend(rows_of(
            Pattern::node("P* 1NT (2♠)"),
            natural_major_overcall(Suit::Spades),
        ));
    }

    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-competition",
        gate: |_| true,
        entries,
    }
}
