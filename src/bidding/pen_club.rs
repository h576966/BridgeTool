//! PEN-Club — a playable, explicitly provisional strong-club system.
//!
//! The authored book contains the opening structure and the continuations that
//! can be stated from the local PEN-Klövern source without changing their
//! meaning. Every other position falls through to the deterministic
//! [`instinct`][mod@super::instinct] ladder. The neural American/Dutch floor is
//! deliberately not used: its convention-card regime does not describe these
//! artificial openings.

mod competition;
mod constructive;
mod openings;

use super::agreements::Agreements;
use super::common::with_instinct_floor;
use super::rows::compile_into;
use super::{Competitive, Constructive, Defensive, System};

/// Build PEN-Club with its deterministic floor.
#[must_use]
pub fn pen_club(agreements: &Agreements) -> System {
    with_instinct_floor(pen_club_book(agreements), agreements)
}

/// Build PEN-Club on the shipped agreement settings.
#[must_use]
pub fn pen_club_default() -> System {
    pen_club(&Agreements::default())
}

/// Build the authored PEN-Club books without a floor.
///
/// This is the source for the web Book view and for tests that distinguish
/// authored nodes from deterministic-floor positions.
#[must_use]
pub fn pen_club_book(agreements: &Agreements) -> System {
    let agreements = *agreements;
    let mut constructive = Constructive::new();
    let mut competitive = Competitive::new();
    compile_into(
        &mut constructive.0,
        &agreements,
        &[openings::package(), constructive::package()],
    );
    compile_into(&mut competitive.0, &agreements, &[competition::package()]);
    System::new(constructive, competitive, Defensive::new(), agreements)
}

/// The authored PEN-Club books on shipped settings.
#[must_use]
pub fn pen_club_book_default() -> System {
    pen_club_book(&Agreements::default())
}

#[cfg(test)]
mod tests;
