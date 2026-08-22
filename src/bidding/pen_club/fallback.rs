//! Conservative natural fallback for PEN positions which are still unspecified.

use crate::bidding::agreements::Agreements;
use crate::bidding::array::Logits;
use crate::bidding::constraint::described;
use crate::bidding::context::Context;
use crate::bidding::fallback::{Always, Fallback};
use crate::bidding::inference::relative_of;
use crate::bidding::instinct::instinct;
use crate::bidding::trie::Classifier;
use crate::bidding::{Rules, System};
use contract_bridge::auction::{Call, RelativeVulnerability};
use contract_bridge::{Bid, Hand, Rank, Strain, Suit, eval};
use std::cmp::Reverse;
use std::sync::Arc;

use super::slam::{CONTROL, RKCB};

#[derive(Clone)]
struct PenSafeNatural {
    natural: Arc<Rules>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SlamAction {
    Control(Bid),
    Keycard(Bid),
    Signoff(Bid),
}

const SLAM_SEARCH_POINTS: u8 = 29;

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

fn our_indexed_bids(context: &Context<'_>) -> Vec<(usize, Bid)> {
    let our_parity = context.auction().len() % 2;
    context
        .auction()
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == our_parity)
        .filter_map(|(index, call)| match call {
            Call::Bid(bid) => Some((index, *bid)),
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
    if (opening == bid(1, Strain::Diamonds)
        && [
            bid(4, Strain::Clubs),
            bid(4, Strain::Diamonds),
            bid(4, Strain::Hearts),
        ]
        .contains(&response))
        || (opening == bid(1, Strain::Hearts)
            && [bid(4, Strain::Clubs), bid(4, Strain::Diamonds)].contains(&response))
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
    if let Some(trump) = publicly_agreed_trump(context) {
        return context
            .last_bid()
            .is_none_or(|shown| shown < game_bid(trump));
    }
    context.last_bid().is_none_or(|bid| match bid.strain {
        Strain::Clubs | Strain::Diamonds => bid.level.get() < 5,
        Strain::Hearts | Strain::Spades => bid.level.get() < 4,
        Strain::Notrump => bid.level.get() < 3,
    })
}

fn game_bid(suit: Suit) -> Bid {
    let level = match suit {
        Suit::Clubs | Suit::Diamonds => 5,
        Suit::Hearts | Suit::Spades => 4,
    };
    bid(level, Strain::from(suit))
}

fn cheapest_bid(context: &Context<'_>, suit: Suit) -> Option<Bid> {
    context
        .min_level(Strain::from(suit))
        .map(|level| bid(level.get(), Strain::from(suit)))
}

fn call_available(context: &Context<'_>, candidate: Bid) -> bool {
    context
        .min_level(candidate.strain)
        .is_some_and(|minimum| minimum <= candidate.level)
}

fn publicly_agreed_trump(context: &Context<'_>) -> Option<Suit> {
    let bids = our_bids(context);
    if let Some((&opening, rest)) = bids.split_first()
        && let Some(&response) = rest.first()
    {
        if opening == bid(1, Strain::Diamonds)
            && [
                bid(4, Strain::Clubs),
                bid(4, Strain::Diamonds),
                bid(4, Strain::Hearts),
            ]
            .contains(&response)
        {
            return Some(Suit::Spades);
        }
        if opening == bid(1, Strain::Hearts)
            && [bid(4, Strain::Clubs), bid(4, Strain::Diamonds)].contains(&response)
        {
            return Some(Suit::Hearts);
        }
    }

    let inferences = context.inferences();
    let partnership_bids = our_indexed_bids(context);
    Suit::ASC
        .into_iter()
        .filter_map(|suit| {
            let mine = inferences.me().length(suit).min;
            let partner = inferences.partner().length(suit).min;
            if mine == 0 || partner == 0 || mine + partner < 8 {
                return None;
            }
            let last = partnership_bids
                .iter()
                .rev()
                .find_map(|(index, shown)| (shown.strain.suit() == Some(suit)).then_some(*index))?;
            Some((last, mine + partner, suit))
        })
        .max_by_key(|&(last, length, suit)| {
            (
                last,
                length,
                matches!(suit, Suit::Hearts | Suit::Spades),
                suit,
            )
        })
        .map(|(_, _, suit)| suit)
}

fn latest_partnership_bid(context: &Context<'_>) -> Option<Bid> {
    our_indexed_bids(context).last().map(|(_, bid)| *bid)
}

fn descriptive_slam_try(context: &Context<'_>) -> bool {
    let partnership_bids = our_indexed_bids(context);
    if partnership_bids.len() < 4 {
        return false;
    }
    let inferences = context.inferences();
    partnership_bids
        .iter()
        .enumerate()
        .skip(3)
        .any(|(_, &(index, shown))| {
            shown < bid(3, Strain::Notrump)
                && shown.strain.suit().is_some_and(|suit| {
                    inferences
                        .get(relative_of(context.auction().len(), index))
                        .length(suit)
                        .min
                        >= 4
                })
        })
}

fn general_slam_intent(context: &Context<'_>) -> bool {
    if !context.undisturbed() {
        return false;
    }
    if let Some(trump) = publicly_agreed_trump(context) {
        if latest_partnership_bid(context) == Some(game_bid(trump)) {
            return false;
        }
        let fit_set_below_game = our_indexed_bids(context).len() >= 4
            && our_indexed_bids(context)
                .iter()
                .rev()
                .find_map(|(_, shown)| (shown.strain.suit() == Some(trump)).then_some(*shown))
                .is_some_and(|shown| shown < game_bid(trump));
        if fit_set_below_game {
            return true;
        }
    }
    descriptive_slam_try(context) || context.inferences().control_bid().is_some()
}

fn combined_hcp_reaches_slam_search(hand: Hand, context: &Context<'_>) -> bool {
    let partner = context.inferences().partner().strength;
    let partner_floor = partner.hcp_floor().unwrap_or_else(|| partner.shown_floor());
    u16::from(hcp(hand)) + u16::from(partner_floor) >= u16::from(SLAM_SEARCH_POINTS)
}

fn own_suit_bid_count(context: &Context<'_>, suit: Suit) -> usize {
    let our_lane = context.auction().len() % 4;
    context
        .auction()
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 4 == our_lane)
        .filter(|(_, call)| matches!(call, Call::Bid(shown) if shown.strain.suit() == Some(suit)))
        .count()
}

fn descriptive_call(hand: Hand, context: &Context<'_>) -> Option<Bid> {
    let inferences = context.inferences();
    let three_notrump = bid(3, Strain::Notrump);
    let new_suit = Suit::ASC
        .into_iter()
        .filter(|&suit| {
            hand[suit].len() >= 4
                && inferences.me().length(suit).min < 4
                && inferences.partner().length(suit).min == 0
        })
        .filter_map(|suit| cheapest_bid(context, suit).map(|candidate| (suit, candidate)))
        .filter(|(_, candidate)| *candidate < three_notrump)
        .max_by_key(|&(suit, candidate)| (hand[suit].len(), Reverse(candidate)))
        .map(|(_, candidate)| candidate);
    if new_suit.is_some() {
        return new_suit;
    }

    Suit::ASC
        .into_iter()
        .filter(|&suit| {
            hand[suit].len() >= 6
                && inferences.me().length(suit).min >= 4
                && own_suit_bid_count(context, suit) < 2
        })
        .filter_map(|suit| cheapest_bid(context, suit).map(|candidate| (suit, candidate)))
        .filter(|(_, candidate)| *candidate < three_notrump)
        .max_by_key(|&(suit, candidate)| (hand[suit].len(), Reverse(candidate)))
        .map(|(_, candidate)| candidate)
}

fn fit_setting_call(hand: Hand, context: &Context<'_>) -> Option<Bid> {
    if publicly_agreed_trump(context).is_some() {
        return None;
    }
    let inferences = context.inferences();
    Suit::ASC
        .into_iter()
        .filter_map(|suit| {
            let partner = usize::from(inferences.partner().length(suit).min);
            let total = hand[suit].len() + partner;
            if partner == 0 || total < 8 {
                return None;
            }
            let candidate = cheapest_bid(context, suit)?;
            (candidate < game_bid(suit)).then_some((total, suit, candidate))
        })
        .max_by_key(|&(total, suit, candidate)| {
            (
                total,
                matches!(suit, Suit::Hearts | Suit::Spades),
                Reverse(candidate),
            )
        })
        .map(|(_, _, candidate)| candidate)
}

fn selected_natural_slam_call(hand: Hand, context: &Context<'_>) -> Option<Bid> {
    if !context.undisturbed()
        || !known_game_force(hand, context)
        || !below_game(context)
        || context
            .auction()
            .iter()
            .any(|call| matches!(call, Call::Bid(shown) if *shown == bid(4, Strain::Notrump)))
        || publicly_agreed_trump(context).is_some()
    {
        return None;
    }
    let active = general_slam_intent(context);
    if !active && !combined_hcp_reaches_slam_search(hand, context) {
        return None;
    }
    fit_setting_call(hand, context).or_else(|| descriptive_call(hand, context))
}

fn has_control(hand: Hand, context: &Context<'_>, suit: Suit) -> bool {
    let holding = hand[suit];
    holding.contains(Rank::A)
        || holding.contains(Rank::K)
        || (holding.len() <= 1 && context.inferences().partner().length(suit).min == 0)
}

fn shown_controls(context: &Context<'_>, trump: Suit) -> [bool; 4] {
    let partnership_bids = our_indexed_bids(context);
    let fit_index = partnership_bids
        .iter()
        .rev()
        .find_map(|(index, shown)| (shown.strain.suit() == Some(trump)).then_some(*index));
    let mut controls = [false; 4];
    let Some(fit_index) = fit_index else {
        return controls;
    };
    for (index, shown) in partnership_bids {
        if index <= fit_index || shown.level.get() < 3 {
            continue;
        }
        if let Some(suit) = shown.strain.suit()
            && suit != trump
        {
            controls[suit as usize] = true;
        }
    }
    controls
}

fn selected_slam_action(hand: Hand, context: &Context<'_>) -> Option<SlamAction> {
    if !context.undisturbed()
        || !general_slam_intent(context)
        || context
            .auction()
            .iter()
            .any(|call| matches!(call, Call::Bid(shown) if *shown == bid(4, Strain::Notrump)))
    {
        return None;
    }
    let trump = publicly_agreed_trump(context)?;
    let shown = shown_controls(context, trump);
    let all_controlled = Suit::ASC
        .into_iter()
        .filter(|&suit| suit != trump)
        .all(|suit| shown[suit as usize] || has_control(hand, context, suit));
    let keycard = bid(4, Strain::Notrump);
    if all_controlled && call_available(context, keycard) {
        return Some(SlamAction::Keycard(keycard));
    }

    let signoff = game_bid(trump);
    let control = Suit::ASC
        .into_iter()
        .filter(|&suit| suit != trump && !shown[suit as usize] && has_control(hand, context, suit))
        .filter_map(|suit| cheapest_bid(context, suit))
        .filter(|candidate| candidate.level.get() >= 3 && *candidate < signoff)
        .min();
    if let Some(control) = control {
        return Some(SlamAction::Control(control));
    }
    call_available(context, signoff).then_some(SlamAction::Signoff(signoff))
}

fn natural_slam_search_rules() -> Rules {
    let mut rules = Rules::new();
    for level in 1..=5 {
        for strain in Strain::ASC {
            let candidate = bid(level, strain);
            if let Some(suit) = strain.suit() {
                rules = rules.rule(
                    candidate,
                    800,
                    described(
                        format!("a natural descriptive slam try or fit-setting {suit} bid"),
                        move |hand: Hand, context: &Context<'_>| {
                            selected_natural_slam_call(hand, context) == Some(candidate)
                        },
                    ),
                );
            }
        }
    }
    rules
}

fn slam_control_rules() -> Rules {
    let mut rules = Rules::new();
    for level in 1..=5 {
        for strain in Strain::ASC {
            let candidate = bid(level, strain);
            if let Some(suit) = strain.suit() {
                if level >= 3 {
                    rules = rules
                        .rule(
                            candidate,
                            900,
                            described(
                                format!("a mixed first- or second-round {suit} control"),
                                move |hand: Hand, context: &Context<'_>| {
                                    selected_slam_action(hand, context)
                                        == Some(SlamAction::Control(candidate))
                                },
                            ),
                        )
                        .alert(CONTROL);
                }
                rules = rules.rule(
                    candidate,
                    850,
                    described(
                        format!("a return to {candidate} declining further slam cooperation"),
                        move |hand: Hand, context: &Context<'_>| {
                            selected_slam_action(hand, context)
                                == Some(SlamAction::Signoff(candidate))
                        },
                    ),
                );
            }
        }
    }
    let keycard = bid(4, Strain::Notrump);
    rules
        .rule(
            keycard,
            950,
            described(
                "RKCB after every untested side suit is controlled",
                move |hand: Hand, context: &Context<'_>| {
                    selected_slam_action(hand, context) == Some(SlamAction::Keycard(keycard))
                },
            ),
        )
        .alert(RKCB)
}

fn face_agreed_trump(context: &Context<'_>) -> Option<Suit> {
    let partnership_bids = our_indexed_bids(context);
    let (_, tail) = partnership_bids.split_first()?;
    Suit::ASC
        .into_iter()
        .filter_map(|suit| {
            let mut lanes = [false; 2];
            let mut last = None;
            for &(index, shown) in tail {
                if shown.strain.suit() == Some(suit) {
                    lanes[usize::from(index % 4 != context.auction().len() % 4)] = true;
                    last = Some(index);
                }
            }
            (lanes[0] && lanes[1]).then_some((last?, suit))
        })
        .max_by_key(|&(last, suit)| (last, matches!(suit, Suit::Hearts | Suit::Spades), suit))
        .map(|(_, suit)| suit)
}

fn face_general_slam_intent(context: &Context<'_>, trump: Suit) -> bool {
    let partnership_bids = our_indexed_bids(context);
    if partnership_bids.len() < 4 || latest_partnership_bid(context) == Some(game_bid(trump)) {
        return false;
    }
    let descriptive = partnership_bids
        .iter()
        .skip(3)
        .any(|(_, shown)| shown.strain.is_suit() && *shown < bid(3, Strain::Notrump));
    let fit_below_game = partnership_bids
        .iter()
        .rev()
        .find_map(|(_, shown)| (shown.strain.suit() == Some(trump)).then_some(*shown))
        .is_some_and(|shown| shown < game_bid(trump));
    descriptive || fit_below_game
}

fn slam_control_position(context: &Context<'_>) -> bool {
    face_agreed_trump(context).is_some_and(|trump| face_general_slam_intent(context, trump))
        && !context
            .auction()
            .iter()
            .any(|call| matches!(call, Call::Bid(shown) if *shown == bid(4, Strain::Notrump)))
}

fn general_rkcb_position(context: &Context<'_>) -> bool {
    context.undisturbed()
        && face_agreed_trump(context).is_some_and(|trump| face_general_slam_intent(context, trump))
        && context
            .auction()
            .iter()
            .any(|call| matches!(call, Call::Bid(shown) if *shown == bid(4, Strain::Notrump)))
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
        || general_slam_intent(context)
}

fn above_game(bid: Bid) -> bool {
    match bid.strain {
        Strain::Clubs | Strain::Diamonds => bid.level.get() > 5,
        Strain::Hearts | Strain::Spades => bid.level.get() > 4,
        Strain::Notrump => bid.level.get() > 3,
    }
}

fn add_legal_game_signoff(logits: &mut Logits, context: &Context<'_>) {
    if let Some(trump) = publicly_agreed_trump(context) {
        let target = game_bid(trump);
        if call_available(context, target) {
            logits.0[Call::Bid(target)] = 0.0;
            return;
        }
    }
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

fn competitive_major_game_allowed(hand: Hand, context: &Context<'_>, trump: Suit) -> bool {
    if context.undisturbed() || known_game_force(hand, context) {
        return true;
    }
    let inferences = context.inferences();
    let partner = inferences.partner();
    let fit = hand[trump].len() as u8 + partner.length(trump).min;
    let combined_hcp = hcp(hand).saturating_add(partner.strength.hcp.min);
    let vul = context.vul();
    let favorable =
        !vul.contains(RelativeVulnerability::WE) && vul.contains(RelativeVulnerability::THEY);
    (fit >= 8 && combined_hcp >= 25) || (fit >= 9 && favorable)
}

impl Classifier for PenSafeNatural {
    fn classify(&self, hand: Hand, context: &Context<'_>) -> Logits {
        // PEN owns its opening structure completely. The natural ladder is a
        // continuation floor, not an alternative opening system: if none of
        // PEN's authored openings accepts the hand, the hand must pass.
        if context.opening_bid().is_none() {
            let mut logits = Logits::new();
            logits.0[Call::Pass] = 0.0;
            return logits;
        }

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

        for suit in [Suit::Hearts, Suit::Spades] {
            if !competitive_major_game_allowed(hand, context, suit) {
                logits.0[Call::Bid(game_bid(suit))] = f32::NEG_INFINITY;
            }
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
        natural: Arc::new(instinct(agreements).chain(natural_slam_search_rules())),
    });
    let rkcb = Fallback::Classify(Arc::new(instinct(agreements)));
    let controls = Fallback::Classify(Arc::new(slam_control_rules()));
    let fallback = Fallback::Classify(floor);
    system.constructive.fallback_at(
        &[],
        |context: &Context<'_>, _: &[Call]| general_rkcb_position(context),
        rkcb.clone(),
    );
    system.constructive.fallback_at(
        &[],
        |context: &Context<'_>, _: &[Call]| slam_control_position(context),
        controls.clone(),
    );
    system
        .constructive
        .fallback_at(&[], Always, fallback.clone());
    system.competitive.fallback_at(
        &[],
        |context: &Context<'_>, _: &[Call]| general_rkcb_position(context),
        rkcb.clone(),
    );
    system.competitive.fallback_at(
        &[],
        |context: &Context<'_>, _: &[Call]| slam_control_position(context),
        controls.clone(),
    );
    system
        .competitive
        .fallback_at(&[], Always, fallback.clone());
    system.defensive.fallback_at(
        &[],
        |context: &Context<'_>, _: &[Call]| general_rkcb_position(context),
        rkcb,
    );
    system.defensive.fallback_at(
        &[],
        |context: &Context<'_>, _: &[Call]| slam_control_position(context),
        controls,
    );
    system.defensive.fallback_at(&[], Always, fallback);
    system
}
