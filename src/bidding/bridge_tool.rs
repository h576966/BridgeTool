//! Pure opening-bid audit for the proposed BridgeTool system.
//!
//! This module classifies literal opening eligibility without constructing a
//! Pons [Rules](super::Rules) table. It is deliberately separate from the
//! shipped American system while opening priorities and meanings remain under
//! review.

use core::fmt;

use contract_bridge::eval;
use contract_bridge::{Hand, Suit};

/// An opening call implemented by the first BridgeTool audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opening {
    /// 1♣: 16+ HCP, except the 16–19 HCP strong-club 1♠ shape.
    OneClub,
    /// 1♦: 10–15 HCP, 4+ spades, and unbalanced.
    OneDiamond,
    /// 1♥: 10–15 HCP, 4+ hearts, fewer than four spades, and unbalanced.
    OneHeart,
    /// 1♠: either a 10–15 HCP minor two-suiter or the 16–19 strong-club shape.
    OneSpade,
    /// 1NT: 12–15 HCP with one of the explicitly permitted shapes.
    OneNotrump,
    /// 2♣: the basic 10–15 HCP single-suited club case.
    TwoClubs,
    /// 2♦: the basic 10–15 HCP single-suited diamond case.
    TwoDiamonds,
}

impl Opening {
    /// Every opening implemented by this audit, in call order.
    pub const ALL: [Self; 7] = [
        Self::OneClub,
        Self::OneDiamond,
        Self::OneHeart,
        Self::OneSpade,
        Self::OneNotrump,
        Self::TwoClubs,
        Self::TwoDiamonds,
    ];
}

impl fmt::Display for Opening {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::OneClub => "1♣",
            Self::OneDiamond => "1♦",
            Self::OneHeart => "1♥",
            Self::OneSpade => "1♠",
            Self::OneNotrump => "1NT",
            Self::TwoClubs => "2♣",
            Self::TwoDiamonds => "2♦",
        })
    }
}

/// Objective information used by the audit, without adjusted points or
/// subjective suit-quality judgments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandFacts {
    /// Raw Milton Work high-card points for the complete hand.
    pub hcp: u8,
    /// Suit lengths in [Suit::ASC] order: clubs, diamonds, hearts, spades.
    pub suit_lengths: [u8; 4],
    /// Raw HCP per suit in [Suit::ASC] order.
    pub suit_hcp: [u8; 4],
    /// Suit lengths sorted from longest to shortest.
    pub shape: [u8; 4],
    /// Suits containing exactly one card.
    pub singletons: Vec<Suit>,
    /// Suits containing no cards.
    pub voids: Vec<Suit>,
}

impl HandFacts {
    /// Extract the objective audit facts from a hand.
    #[must_use]
    pub fn from_hand(hand: Hand) -> Self {
        let suit_lengths = Suit::ASC.map(|suit| hand[suit].len() as u8);
        let suit_hcp = Suit::ASC.map(|suit| eval::hcp::<u8>(hand[suit]));
        let mut shape = suit_lengths;
        shape.sort_unstable_by(|a, b| b.cmp(a));

        Self {
            hcp: suit_hcp.iter().sum(),
            suit_lengths,
            suit_hcp,
            shape,
            singletons: Suit::ASC
                .into_iter()
                .filter(|&suit| suit_lengths[suit as usize] == 1)
                .collect(),
            voids: Suit::ASC
                .into_iter()
                .filter(|&suit| suit_lengths[suit as usize] == 0)
                .collect(),
        }
    }

    /// The number of cards held in the given suit.
    #[must_use]
    pub const fn length(&self, suit: Suit) -> u8 {
        self.suit_lengths[suit as usize]
    }

    /// Whether the hand satisfies the audit's general unbalanced definition.
    #[must_use]
    pub const fn is_unbalanced(&self) -> bool {
        self.shape[0] + self.shape[1] >= 9 || self.shape[3] <= 1
    }

    /// Whether the hand is one of the ordinary 4333, 4432, or 5332 shapes.
    #[must_use]
    pub const fn is_ordinary_balanced(&self) -> bool {
        matches!(self.shape, [4, 3, 3, 3] | [4, 4, 3, 2] | [5, 3, 3, 2])
    }
}

impl From<Hand> for HandFacts {
    fn from(hand: Hand) -> Self {
        Self::from_hand(hand)
    }
}

impl fmt::Display for HandFacts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} HCP; shape {}-{}-{}-{}; lengths ♣{} ♦{} ♥{} ♠{}; suit HCP ♣{} ♦{} ♥{} ♠{}; singletons {}; voids {}",
            self.hcp,
            self.shape[0],
            self.shape[1],
            self.shape[2],
            self.shape[3],
            self.length(Suit::Clubs),
            self.length(Suit::Diamonds),
            self.length(Suit::Hearts),
            self.length(Suit::Spades),
            self.suit_hcp[Suit::Clubs as usize],
            self.suit_hcp[Suit::Diamonds as usize],
            self.suit_hcp[Suit::Hearts as usize],
            self.suit_hcp[Suit::Spades as usize],
            suit_list(&self.singletons),
            suit_list(&self.voids),
        )
    }
}

/// Result of applying only the explicitly stated opening priorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpeningSelection {
    /// One opening was selected.
    Selected(Opening),
    /// No implemented opening matched.
    NoMatch,
    /// Several openings remain and require a system decision.
    Ambiguous(Vec<Opening>),
}

/// Return every implemented opening whose literal conditions the hand meets.
#[must_use]
pub fn eligible_openings(hand: Hand) -> Vec<Opening> {
    let facts = HandFacts::from(hand);
    Opening::ALL
        .into_iter()
        .filter(|&opening| is_eligible(opening, &facts))
        .collect()
}

/// Select an opening only where the audit defines an explicit resolution.
#[must_use]
pub fn select_opening(hand: Hand) -> OpeningSelection {
    let candidates = eligible_openings(hand);
    if candidates.contains(&Opening::OneNotrump) {
        OpeningSelection::Selected(Opening::OneNotrump)
    } else {
        match candidates.as_slice() {
            [] => OpeningSelection::NoMatch,
            [opening] => OpeningSelection::Selected(*opening),
            _ => OpeningSelection::Ambiguous(candidates),
        }
    }
}

/// Return shape-only candidates for the unresolved 6–4 minor exception.
///
/// No HCP or suit-quality condition is added here because none has been
/// defined for this diagnostic. A returned call is not an eligible opening.
#[must_use]
pub fn minor_exception_candidates(hand: Hand) -> Vec<Opening> {
    let facts = HandFacts::from(hand);
    [
        (Suit::Clubs, Suit::Diamonds, Opening::TwoClubs),
        (Suit::Diamonds, Suit::Clubs, Opening::TwoDiamonds),
    ]
    .into_iter()
    .filter_map(|(minor, other_minor, opening)| {
        (facts.length(minor) >= 6
            && facts.length(other_minor) == 4
            && facts.length(Suit::Hearts) < 4
            && facts.length(Suit::Spades) < 4)
            .then_some(opening)
    })
    .collect()
}

fn is_eligible(opening: Opening, facts: &HandFacts) -> bool {
    let hcp_10_to_15 = (10..=15).contains(&facts.hcp);
    match opening {
        Opening::OneClub => facts.hcp >= 16 && !is_strong_club_one_spade(facts),
        Opening::OneDiamond => {
            hcp_10_to_15 && facts.length(Suit::Spades) >= 4 && facts.is_unbalanced()
        }
        Opening::OneHeart => {
            hcp_10_to_15
                && facts.length(Suit::Hearts) >= 4
                && facts.length(Suit::Spades) < 4
                && facts.is_unbalanced()
        }
        Opening::OneSpade => {
            (hcp_10_to_15 && facts.length(Suit::Clubs) >= 4 && facts.length(Suit::Diamonds) >= 4)
                || is_strong_club_one_spade(facts)
        }
        Opening::OneNotrump => is_one_notrump(facts),
        Opening::TwoClubs => hcp_10_to_15 && is_basic_minor(facts, Suit::Clubs),
        Opening::TwoDiamonds => hcp_10_to_15 && is_basic_minor(facts, Suit::Diamonds),
    }
}

fn is_one_notrump(facts: &HandFacts) -> bool {
    (12..=15).contains(&facts.hcp)
        && facts.length(Suit::Hearts) >= 3
        && facts.length(Suit::Spades) >= 3
        && (facts.is_ordinary_balanced()
            || (facts.shape == [4, 4, 4, 1]
                && (facts.length(Suit::Clubs) == 1 || facts.length(Suit::Diamonds) == 1)))
}

fn is_strong_club_one_spade(facts: &HandFacts) -> bool {
    (16..=19).contains(&facts.hcp)
        && facts.length(Suit::Clubs) >= 5
        && facts.is_unbalanced()
        && [Suit::Diamonds, Suit::Hearts, Suit::Spades]
            .into_iter()
            .all(|suit| facts.length(Suit::Clubs) > facts.length(suit))
}

fn is_basic_minor(facts: &HandFacts, minor: Suit) -> bool {
    facts.length(minor) >= 5
        && Suit::ASC
            .into_iter()
            .all(|suit| suit == minor || facts.length(suit) <= 3)
}

fn suit_list(suits: &[Suit]) -> String {
    if suits.is_empty() {
        "-".to_owned()
    } else {
        suits.iter().map(ToString::to_string).collect()
    }
}

#[cfg(test)]
mod tests;
