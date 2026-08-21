//! PEN-Club's confirmed natural defensive foundation.

use crate::bidding::agreements::Agreements;
use crate::bidding::constraint::{
    Cons, Constraint, at_least_as_long, balanced, described, hcp, len, longer_suit, min_level_is,
    partner_shown_len, short_in_their_suits, stopper_in, top_honors, unbid_support,
};
use crate::bidding::inference::{Relative, relative_of};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Hand, Strain, Suit};

const TAKEOUT: Alert = Alert("pen:takeout-double");
const COOPERATIVE: Alert = Alert("pen:cooperative-double");
const MICHAELS: Alert = Alert("pen:michaels");
const UNUSUAL: Alert = Alert("pen:unusual-two-notrump");
const ADVANCE_CUE: Alert = Alert("pen:overcall-advance-cue");
const RESPONSIVE: Alert = Alert("pen:responsive-double");
const LANDY: Alert = Alert("pen:landy");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

/// Require the opponent's declared system to show cards in the suit it named.
/// This keeps PEN's natural defenses off artificial strong-club/relay openings.
fn natural_suit_opening(suit: Suit) -> Cons<impl Constraint + Clone> {
    described(
        format!("the opponents disclose a natural {suit} opening"),
        move |_: Hand, context: &crate::bidding::context::Context<'_>| {
            let Some(index) = context
                .auction()
                .iter()
                .position(|call| *call != Call::Pass)
            else {
                return false;
            };
            let relative = relative_of(context.auction().len(), index);
            let inferences = context.inferences();
            let shown = inferences.get(relative);
            matches!(relative, Relative::Lho | Relative::Rho) && shown.length(suit).min >= 3
        },
    )
}

fn natural_notrump_opening() -> Cons<impl Constraint + Clone> {
    described(
        "the opponents disclose a natural notrump opening",
        |_: Hand, context: &crate::bidding::context::Context<'_>| {
            let Some(index) = context
                .auction()
                .iter()
                .position(|call| *call != Call::Pass)
            else {
                return false;
            };
            let relative = relative_of(context.auction().len(), index);
            let inferences = context.inferences();
            let shown = inferences.get(relative);
            matches!(relative, Relative::Lho | Relative::Rho)
                && shown.strength.hcp.min >= 10
                && Suit::ASC
                    .into_iter()
                    .all(|suit| shown.length(suit).min >= 2)
        },
    )
}

fn direct_suit_defense(opened: Suit) -> Rules {
    let natural = natural_suit_opening(opened);
    let mut rules = Rules::new()
        .rule(
            Call::Double,
            500,
            natural.clone()
                & ((hcp(12..=17) & !balanced() & short_in_their_suits() & unbid_support(1))
                    | hcp(18..)),
        )
        .alert(TAKEOUT)
        .rule(
            bid(1, Strain::Notrump),
            440,
            natural.clone() & hcp(15..=18) & balanced() & stopper_in(opened),
        );

    for suit in Suit::ASC {
        if suit == opened {
            continue;
        }
        rules = rules
            .rule(
                bid(1, Strain::from(suit)),
                350 + i16::from(Strain::from(suit).is_major()) * 10,
                natural.clone()
                    & min_level_is(1, Strain::from(suit))
                    & len(suit, 5..)
                    & hcp(8..=17),
            )
            .rule(
                bid(2, Strain::from(suit)),
                350 + i16::from(Strain::from(suit).is_major()) * 10,
                natural.clone()
                    & min_level_is(2, Strain::from(suit))
                    & (len(suit, 6..) | (len(suit, 5..) & top_honors(suit, 2..)))
                    & hcp(10..=17),
            );
        for level in 2..=3u8 {
            rules = rules.rule(
                bid(level, Strain::from(suit)),
                250,
                natural.clone()
                    & len(suit, 6..)
                    & hcp(5..=9)
                    & min_level_is(level - 1, Strain::from(suit)),
            );
        }
    }

    if matches!(opened, Suit::Clubs | Suit::Diamonds) {
        rules = rules
            .rule(
                bid(2, Strain::from(opened)),
                480,
                natural.clone()
                    & len(Suit::Hearts, 5..)
                    & len(Suit::Spades, 5..)
                    & (hcp(..=11) | hcp(16..)),
            )
            .alert(MICHAELS);
    } else {
        let other_major = if opened == Suit::Hearts {
            Suit::Spades
        } else {
            Suit::Hearts
        };
        rules = rules
            .rule(
                bid(2, Strain::from(opened)),
                480,
                natural.clone()
                    & len(other_major, 5..)
                    & (len(Suit::Clubs, 5..) | len(Suit::Diamonds, 5..))
                    & (hcp(..=11) | hcp(16..)),
            )
            .alert(MICHAELS);
    }

    let (low, high) = match opened {
        Suit::Clubs => (Suit::Diamonds, Suit::Hearts),
        Suit::Diamonds => (Suit::Clubs, Suit::Hearts),
        Suit::Hearts | Suit::Spades => (Suit::Clubs, Suit::Diamonds),
    };
    rules
        .rule(
            bid(2, Strain::Notrump),
            470,
            natural & len(low, 5..) & len(high, 5..) & (hcp(..=11) | hcp(16..)),
        )
        .alert(UNUSUAL)
}

fn overcall_advances(opened: Suit, overcall: Bid) -> Rules {
    let Some(our) = overcall.strain.suit() else {
        return Rules::new();
    };
    let mut rules = Rules::new()
        .rule(bid(2, Strain::from(opened)), 500, hcp(10..) & len(our, 3..))
        .alert(ADVANCE_CUE)
        .rule(
            bid(overcall.level.get() + 2, overcall.strain),
            350,
            hcp(..=8) & len(our, 4..),
        )
        .rule(
            bid(overcall.level.get() + 1, overcall.strain),
            300,
            hcp(6..=9) & len(our, 3..),
        )
        .rule(
            bid(3, Strain::Notrump),
            320,
            hcp(13..) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(2, Strain::Notrump),
            260,
            hcp(11..=12) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(1, Strain::Notrump),
            220,
            hcp(6..=10) & balanced() & stopper_in(opened),
        );
    for suit in Suit::ASC {
        if suit == opened || suit == our {
            continue;
        }
        for level in 1..=3u8 {
            rules = rules
                .rule(
                    bid(level + 1, Strain::from(suit)),
                    320,
                    hcp(10..=12)
                        & len(suit, 6..)
                        & top_honors(suit, 2..)
                        & min_level_is(level, Strain::from(suit)),
                )
                .rule(
                    bid(level, Strain::from(suit)),
                    280,
                    hcp(8..) & len(suit, 5..) & min_level_is(level, Strain::from(suit)),
                );
        }
    }
    rules
}

fn takeout_advances(opened: Suit) -> Rules {
    let mut rules = Rules::new()
        .rule(
            Call::Pass,
            600,
            hcp(6..) & len(opened, 5..) & top_honors(opened, 2..),
        )
        .rule(bid(2, Strain::from(opened)), 500, hcp(13..))
        .alert(ADVANCE_CUE)
        .rule(
            bid(3, Strain::Notrump),
            450,
            hcp(13..) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(2, Strain::Notrump),
            380,
            hcp(11..=12) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(1, Strain::Notrump),
            320,
            hcp(6..=10) & balanced() & stopper_in(opened),
        );
    for suit in Suit::ASC {
        if suit == opened {
            continue;
        }
        for level in 1..=3u8 {
            rules = rules
                .rule(
                    bid(level, Strain::from(suit)),
                    250,
                    hcp(..=8) & len(suit, 4..) & min_level_is(level, Strain::from(suit)),
                )
                .rule(
                    bid(level + 1, Strain::from(suit)),
                    300,
                    hcp(9..=11) & len(suit, 4..) & min_level_is(level, Strain::from(suit)),
                );
        }
    }
    rules
}

fn balancing_defense(opened: Suit) -> Rules {
    let natural = natural_suit_opening(opened);
    let mut rules = Rules::new()
        .rule(
            Call::Double,
            500,
            natural.clone() & hcp(9..) & short_in_their_suits() & unbid_support(1),
        )
        .alert(TAKEOUT)
        .rule(
            bid(1, Strain::Notrump),
            450,
            natural.clone() & hcp(11..=14) & balanced() & stopper_in(opened),
        );
    for suit in Suit::ASC {
        if suit == opened {
            continue;
        }
        for level in 1..=3u8 {
            rules = rules
                .rule(
                    bid(level + 1, Strain::from(suit)),
                    340,
                    natural.clone()
                        & hcp(9..=14)
                        & len(suit, 6..)
                        & top_honors(suit, 2..)
                        & min_level_is(level, Strain::from(suit)),
                )
                .rule(
                    bid(level, Strain::from(suit)),
                    300,
                    natural.clone()
                        & hcp(5..=17)
                        & len(suit, 5..)
                        & min_level_is(level, Strain::from(suit)),
                );
        }
    }
    rules
}

fn weak_two_defense(opened: Suit) -> Rules {
    let natural = natural_suit_opening(opened);
    let mut rules = Rules::new()
        .rule(
            Call::Double,
            500,
            natural.clone() & hcp(12..) & short_in_their_suits() & unbid_support(1),
        )
        .alert(TAKEOUT)
        .rule(
            bid(2, Strain::Notrump),
            450,
            natural.clone() & hcp(15..=18) & balanced() & stopper_in(opened),
        );
    for suit in Suit::ASC {
        if suit == opened {
            continue;
        }
        for level in 2..=4u8 {
            rules = rules.rule(
                bid(level, Strain::from(suit)),
                300,
                natural.clone()
                    & hcp(10..=17)
                    & (len(suit, 6..) | (len(suit, 5..) & top_honors(suit, 2..)))
                    & min_level_is(level, Strain::from(suit)),
            );
        }
    }
    rules
}

fn higher_preempt_defense(level: u8, opened: Suit) -> Rules {
    let natural = natural_suit_opening(opened);
    let mut rules = if level == 4 && opened == Suit::Spades {
        Rules::new()
            .rule(Call::Double, 500, natural.clone() & hcp(12..))
            .alert(COOPERATIVE)
    } else {
        Rules::new()
            .rule(
                Call::Double,
                500,
                natural.clone() & hcp(12..) & short_in_their_suits() & unbid_support(1),
            )
            .alert(TAKEOUT)
    };
    for suit in Suit::ASC {
        if suit == opened {
            continue;
        }
        for landing in level..=5 {
            rules = rules.rule(
                bid(landing, Strain::from(suit)),
                300,
                natural.clone()
                    & hcp(10..)
                    & (len(suit, 6..) | (len(suit, 5..) & top_honors(suit, 2..)))
                    & min_level_is(landing, Strain::from(suit)),
            );
        }
    }
    rules
}

fn doubler_rebid(opened: Suit, response: Suit) -> Rules {
    let mut rules = Rules::new()
        .rule(
            bid(3, Strain::Notrump),
            360,
            hcp(18..) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(2, Strain::from(response)),
            340,
            hcp(18..) & len(response, 4..) & min_level_is(2, Strain::from(response)),
        );
    for suit in Suit::ASC {
        if suit == opened || suit == response {
            continue;
        }
        for level in 1..=3 {
            rules = rules.rule(
                bid(level, Strain::from(suit)),
                330,
                hcp(18..) & len(suit, 5..) & min_level_is(level, Strain::from(suit)),
            );
        }
    }
    rules
}

fn notrump_defense() -> Rules {
    let natural = natural_notrump_opening();
    Rules::new()
        .rule(Call::Double, 500, natural.clone() & hcp(15..))
        .rule(
            bid(2, Strain::Clubs),
            450,
            natural.clone()
                & ((len(Suit::Hearts, 5..) & len(Suit::Spades, 4..))
                    | (len(Suit::Hearts, 4..) & len(Suit::Spades, 5..))),
        )
        .alert(LANDY)
        .rule(
            bid(2, Strain::Notrump),
            440,
            natural.clone() & len(Suit::Clubs, 5..) & len(Suit::Diamonds, 5..),
        )
        .alert(UNUSUAL)
        .rule(
            bid(2, Strain::Diamonds),
            300,
            natural.clone() & hcp(8..=17) & len(Suit::Diamonds, 5..),
        )
        .rule(
            bid(2, Strain::Hearts),
            300,
            natural.clone() & hcp(8..=17) & len(Suit::Hearts, 5..),
        )
        .rule(
            bid(2, Strain::Spades),
            300,
            natural & hcp(8..=17) & len(Suit::Spades, 5..),
        )
}

fn landy_advance() -> Rules {
    Rules::new()
        .rule(
            bid(2, Strain::Spades),
            250,
            longer_suit(Suit::Spades, Suit::Hearts),
        )
        .rule(
            bid(2, Strain::Hearts),
            240,
            at_least_as_long(Suit::Hearts, Suit::Spades),
        )
}

fn michaels_major_ask_answer(other_major: Suit) -> Rules {
    Rules::new()
        .rule(bid(3, Strain::Clubs), 250, len(Suit::Clubs, 5..))
        .rule(bid(3, Strain::Diamonds), 250, len(Suit::Diamonds, 5..))
        .rule(
            bid(3, Strain::from(other_major)),
            100,
            partner_shown_len(other_major, 3..),
        )
}

fn entries(_: &Agreements) -> Vec<Entry> {
    let mut entries = Vec::new();
    for opened in Suit::ASC {
        let opening = bid(1, Strain::from(opened));
        let root = format!("P* ({opening})");
        entries.extend(rows_of(Pattern::node(&root), direct_suit_defense(opened)));
        entries.extend(rows_of(
            Pattern::node(&format!("{root} - -")),
            balancing_defense(opened),
        ));
        entries.extend(rows_of(
            Pattern::node(&format!("{root} X -")),
            takeout_advances(opened),
        ));
        for response in Suit::ASC {
            if response == opened {
                continue;
            }
            let level = if response > opened { 1 } else { 2 };
            entries.extend(rows_of(
                Pattern::node(&format!(
                    "{root} X - {} -",
                    bid(level, Strain::from(response))
                )),
                doubler_rebid(opened, response),
            ));
        }
        for over in Suit::ASC {
            if over == opened {
                continue;
            }
            let level = if over > opened { 1 } else { 2 };
            let overcall = bid(level, Strain::from(over));
            entries.extend(rows_of(
                Pattern::node(&format!("{root} {overcall} -")),
                overcall_advances(opened, overcall),
            ));
        }
        if matches!(opened, Suit::Hearts | Suit::Spades) {
            let other_major = if opened == Suit::Hearts {
                Suit::Spades
            } else {
                Suit::Hearts
            };
            entries.extend(rows_of(
                Pattern::node(&format!("{root} {} - 2NT -", bid(2, Strain::from(opened)))),
                michaels_major_ask_answer(other_major),
            ));
        }
        entries.extend(rows_of(
            Pattern::node(&format!("{root} X ({})", bid(2, Strain::from(opened)))),
            Rules::new()
                .rule(Call::Double, 300, hcp(8..))
                .alert(RESPONSIVE),
        ));
    }

    for opened in [Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        entries.extend(rows_of(
            Pattern::node(&format!("P* ({})", bid(2, Strain::from(opened)))),
            weak_two_defense(opened),
        ));
    }
    for level in [3, 4] {
        for opened in Suit::ASC {
            entries.extend(rows_of(
                Pattern::node(&format!("P* ({})", bid(level, Strain::from(opened)))),
                higher_preempt_defense(level, opened),
            ));
        }
    }
    entries.extend(rows_of(Pattern::node("P* (1NT)"), notrump_defense()));
    entries.extend(rows_of(Pattern::node("P* (1NT) 2♣ -"), landy_advance()));
    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-defense",
        gate: |_| true,
        entries,
    }
}
