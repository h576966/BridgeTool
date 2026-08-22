//! Strength partitions used by PEN-Club's limited suit openings.

use crate::bidding::constraint::{
    Cons, Constraint, has_rank, hcp, longest_two_at_most, they_vulnerable, top_honors, vulnerable,
};
use contract_bridge::{Rank, Suit};

/// The minimum half of a limited suit opening ending at 15 HCP.
///
/// `floor` is 10 for 1♦/1♥/the limited 1♠ branch and 11 for 2♣/2♦.
pub(super) fn limited_minimum(floor: u8) -> Cons<impl Constraint + Clone> {
    hcp(floor..=12) | (hcp(13..=13) & longest_two_at_most(9))
}

/// The maximum half of a limited suit opening ending at 15 HCP.
pub(super) fn limited_maximum() -> Cons<impl Constraint + Clone> {
    (hcp(13..=13) & !longest_two_at_most(9)) | hcp(14..=15)
}

/// PEN's deliberately generous preempt strength and suit-quality floor.
///
/// Normally 5–9 HCP and one of A/K/Q is enough. Vulnerable against
/// non-vulnerable opponents, the floor rises to 7 HCP and a bare queen no
/// longer qualifies: the suit needs A, K, QJ, or QT.
pub(super) fn preempt_strength(suit: Suit) -> Cons<impl Constraint + Clone> {
    let unfavorable = vulnerable() & !they_vulnerable();
    let normal = !unfavorable.clone() & hcp(5..=9) & top_honors(suit, 1..);
    let unfavorable_quality = has_rank(suit, Rank::A)
        | has_rank(suit, Rank::K)
        | (has_rank(suit, Rank::Q) & (has_rank(suit, Rank::J) | has_rank(suit, Rank::T)));
    normal | (unfavorable & hcp(7..=9) & unfavorable_quality)
}
