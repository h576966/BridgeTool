//! Conservative natural fallback for PEN positions which are still unspecified.

use crate::bidding::agreements::Agreements;
use crate::bidding::array::Logits;
use crate::bidding::context::Context;
use crate::bidding::fallback::{Always, Fallback};
use crate::bidding::instinct::instinct;
use crate::bidding::trie::Classifier;
use crate::bidding::{Rules, System};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Hand, Strain, Suit, eval};
use std::sync::Arc;

#[derive(Clone)]
struct PenSafeNatural {
    natural: Arc<Rules>,
}

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

fn hcp(hand: Hand) -> u8 {
    Suit::ASC
        .into_iter()
        .map(|suit| eval::hcp::<u8>(hand[suit]))
        .sum()
}

fn our_bids(context: &Context<'_>) -> Vec<Bid> {
    let our_parity = context.auction().len() % 2;
    context
        .auction()
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == our_parity)
        .filter_map(|(_, call)| match call {
            Call::Bid(bid) => Some(*bid),
            Call::Pass | Call::Double | Call::Redouble => None,
        })
        .collect()
}

fn known_game_force(hand: Hand, context: &Context<'_>) -> bool {
    if !context.we_opened() {
        return false;
    }
    let bids = our_bids(context);
    let Some((&opening, rest)) = bids.split_first() else {
        return false;
    };
    let Some(&response) = rest.first() else {
        return false;
    };

    if opening == bid(1, Strain::Clubs) {
        return ![
            bid(1, Strain::Diamonds),
            bid(2, Strain::Hearts),
            bid(2, Strain::Spades),
        ]
        .contains(&response);
    }
    if matches!(opening.strain, Strain::Diamonds | Strain::Hearts)
        && opening.level.get() == 1
        && [bid(2, Strain::Clubs), bid(2, Strain::Diamonds)].contains(&response)
    {
        return true;
    }
    if opening == bid(1, Strain::Spades) {
        if response == bid(2, Strain::Notrump) {
            return true;
        }
        if response == bid(2, Strain::Spades) && rest.len() >= 2 {
            let answer = rest[1];
            let strong_answer = [
                bid(3, Strain::Diamonds),
                bid(3, Strain::Hearts),
                bid(3, Strain::Spades),
                bid(3, Strain::Notrump),
            ]
            .contains(&answer);
            return strong_answer || hcp(hand) >= 15;
        }
        return false;
    }
    if opening == bid(2, Strain::Diamonds) && response == bid(3, Strain::Clubs) {
        return true;
    }
    if opening != bid(1, Strain::Notrump) || rest.len() < 2 {
        return false;
    }

    let continuation = rest.get(2).copied();
    match response {
        response if response == bid(2, Strain::Clubs) => match continuation {
            None => hcp(hand) >= 13,
            Some(next) if next == bid(2, Strain::Notrump) => hcp(hand) >= 15,
            Some(next) if next == bid(2, Strain::Hearts) => false,
            Some(_) => true,
        },
        response if [bid(2, Strain::Diamonds), bid(2, Strain::Hearts)].contains(&response) => {
            let target = if response == bid(2, Strain::Diamonds) {
                Strain::Hearts
            } else {
                Strain::Spades
            };
            match continuation {
                None => hcp(hand) >= 13,
                Some(next)
                    if next == bid(2, Strain::Notrump)
                        || next.strain == target && next.level.get() == 3 =>
                {
                    hcp(hand) >= 15
                }
                Some(next)
                    if next == bid(3, Strain::Notrump)
                        || next.strain == target && next.level.get() == 4 =>
                {
                    false
                }
                Some(_) => true,
            }
        }
        response if [bid(2, Strain::Spades), bid(2, Strain::Notrump)].contains(&response) => {
            continuation.is_some() || hcp(hand) >= 13
        }
        _ => false,
    }
}

fn below_game(context: &Context<'_>) -> bool {
    context.last_bid().is_none_or(|bid| match bid.strain {
        Strain::Clubs | Strain::Diamonds => bid.level.get() < 5,
        Strain::Hearts | Strain::Spades => bid.level.get() < 4,
        Strain::Notrump => bid.level.get() < 3,
    })
}

fn slam_intent(context: &Context<'_>) -> bool {
    let partnership_bids = our_bids(context);
    let splinter_intent = partnership_bids.get(1).is_some_and(|response| {
        let opening = partnership_bids[0];
        let (is_splinter, signoff) = if opening == bid(1, Strain::Diamonds) {
            (
                [
                    bid(4, Strain::Clubs),
                    bid(4, Strain::Diamonds),
                    bid(4, Strain::Hearts),
                ]
                .contains(response),
                bid(4, Strain::Spades),
            )
        } else if opening == bid(1, Strain::Hearts) {
            (
                [bid(4, Strain::Clubs), bid(4, Strain::Diamonds)].contains(response),
                bid(4, Strain::Hearts),
            )
        } else {
            (false, bid(7, Strain::Notrump))
        };
        is_splinter && partnership_bids.last() != Some(&signoff)
    });
    context
        .auction()
        .iter()
        .any(|call| matches!(call, Call::Bid(bid) if bid == &Bid::new(4, Strain::Notrump)))
        || partnership_bids.iter().copied().any(above_game)
        || splinter_intent
}

fn above_game(bid: Bid) -> bool {
    match bid.strain {
        Strain::Clubs | Strain::Diamonds => bid.level.get() > 5,
        Strain::Hearts | Strain::Spades => bid.level.get() > 4,
        Strain::Notrump => bid.level.get() > 3,
    }
}

fn add_legal_game_signoff(logits: &mut Logits, context: &Context<'_>) {
    for target in [
        bid(3, Strain::Notrump),
        bid(4, Strain::Hearts),
        bid(4, Strain::Spades),
        bid(5, Strain::Clubs),
        bid(5, Strain::Diamonds),
    ] {
        if context
            .min_level(target.strain)
            .is_some_and(|minimum| minimum <= target.level)
        {
            logits.0[Call::Bid(target)] = 0.0;
            return;
        }
    }
}

impl Classifier for PenSafeNatural {
    fn classify(&self, hand: Hand, context: &Context<'_>) -> Logits {
        let mut logits = self.natural.classify(hand, context);
        // All PEN Doubles and Redoubles are authored at the positions where
        // their meaning is known. An unspecified tail must not inherit an
        // American penalty/cooperative interpretation from the natural ladder.
        logits.0[Call::Double] = f32::NEG_INFINITY;
        logits.0[Call::Redouble] = f32::NEG_INFINITY;
        let force = known_game_force(hand, context) && below_game(context);
        if force {
            logits.0[Call::Pass] = f32::NEG_INFINITY;
        }

        if !slam_intent(context) {
            logits.0[Call::Bid(bid(4, Strain::Notrump))] = f32::NEG_INFINITY;
            for level in 1..=7 {
                for strain in Strain::ASC {
                    let candidate = bid(level, strain);
                    if above_game(candidate) {
                        logits.0[Call::Bid(candidate)] = f32::NEG_INFINITY;
                    }
                }
            }
        }

        if force && !logits.has_mass() {
            add_legal_game_signoff(&mut logits, context);
        }
        logits
    }
}

pub(super) fn attach(mut system: System, agreements: &Agreements) -> System {
    let floor = Arc::new(PenSafeNatural {
        natural: Arc::new(instinct(agreements)),
    });
    let fallback = Fallback::Classify(floor);
    system
        .constructive
        .fallback_at(&[], Always, fallback.clone());
    system
        .competitive
        .fallback_at(&[], Always, fallback.clone());
    system.defensive.fallback_at(&[], Always, fallback);
    system
}
