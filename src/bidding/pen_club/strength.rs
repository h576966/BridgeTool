//! Strength partitions used by PEN-Club's limited suit openings.

use crate::bidding::constraint::{Cons, Constraint, hcp, longest_two_at_most};

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
