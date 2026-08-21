//! PEN-Club — a playable, explicitly provisional strong-club system.
//!
//! The authored book contains the opening structure and the continuations that
//! can be stated from the local PEN-Klövern source without changing their
//! meaning. Every other position falls through to a PEN-only safe wrapper around
//! the natural [`instinct`][mod@super::instinct] ladder. The wrapper preserves
//! known game forces and cannot invent a slam trigger. The neural American/Dutch
//! floor is deliberately not used: its convention-card regime does not describe
//! these artificial openings.

mod competition;
mod constructive;
mod defense;
mod fallback;
mod one_notrump;
mod one_spade;
mod openings;
mod quantitative;
mod slam;
mod strength;

use super::agreements::Agreements;
use super::rows::compile_into;
use super::{Competitive, Constructive, Defensive, System};

/// Build PEN-Club with its PEN-safe natural fallback.
#[must_use]
pub fn pen_club(agreements: &Agreements) -> System {
    fallback::attach(pen_club_book(agreements), agreements)
}

/// Build PEN-Club on the shipped agreement settings.
#[must_use]
pub fn pen_club_default() -> System {
    pen_club(&Agreements::default())
}

/// Build the authored PEN-Club books without a fallback.
///
/// This is the source for the web Book view and for tests that distinguish
/// authored nodes from fallback positions.
#[must_use]
pub fn pen_club_book(agreements: &Agreements) -> System {
    let agreements = *agreements;
    let mut constructive = Constructive::new();
    let mut competitive = Competitive::new();
    let mut defensive = Defensive::new();
    compile_into(
        &mut constructive.0,
        &agreements,
        &[
            openings::package(),
            constructive::package(),
            one_notrump::package(),
            one_spade::package(),
            quantitative::package(),
            slam::package(),
        ],
    );
    compile_into(&mut competitive.0, &agreements, &[competition::package()]);
    compile_into(&mut defensive.0, &agreements, &[defense::package()]);
    System::new(constructive, competitive, defensive, agreements)
}

/// The authored PEN-Club books on shipped settings.
#[must_use]
pub fn pen_club_book_default() -> System {
    pen_club_book(&Agreements::default())
}

#[cfg(test)]
mod tests;
