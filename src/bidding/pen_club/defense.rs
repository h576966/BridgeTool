//! PEN-Club's confirmed natural defensive foundation.

use crate::bidding::agreements::Agreements;
use crate::bidding::constraint::{
    Cons, Constraint, and, at_least_as_long, balanced, described, hcp, len, longer_suit,
    min_level_is, or, partner_shown_len, short_in_their_suits, stopper_in, top_honors,
    unbid_support,
};
use crate::bidding::inference::{Envelope, Relative, relative_of};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Hand, Strain, Suit};

use super::strength::preempt_strength;

const TAKEOUT: Alert = Alert("pen:takeout-double");
const COOPERATIVE: Alert = Alert("pen:cooperative-double");
const BUSINESS: Alert = Alert("pen:business-redouble");
const MICHAELS: Alert = Alert("pen:michaels");
const UNUSUAL: Alert = Alert("pen:unusual-two-notrump");
const ADVANCE_CUE: Alert = Alert("pen:overcall-advance-cue");
const MICHAELS_ASK: Alert = Alert("pen:michaels-minor-ask");
const RESPONSIVE: Alert = Alert("pen:responsive-double");
const LANDY: Alert = Alert("pen:landy");
const STRONG_CLUB_MINORS: Alert = Alert("pen:strong-club-defense-minors");
const STRONG_CLUB_MAJORS: Alert = Alert("pen:strong-club-defense-majors");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

/// Read the opening from its own auction prefix. Later calls may have other
/// plausible meanings, but they must not change what the opening disclosed.
fn opponent_opening_satisfies(
    context: &crate::bidding::context::Context<'_>,
    accepts: impl FnOnce(&Envelope) -> bool,
) -> bool {
    let Some(index) = context
        .auction()
        .iter()
        .position(|call| *call != Call::Pass)
    else {
        return false;
    };
    let relative = relative_of(context.auction().len(), index);
    if !matches!(relative, Relative::Lho | Relative::Rho) {
        return false;
    }

    if let Some(system) = context.own_system() {
        let prefix = &context.auction()[..=index];
        let turns_removed = context.auction().len() - prefix.len();
        let vul = if turns_removed.is_multiple_of(2) {
            context.vul()
        } else {
            crate::bidding::context::flipped(context.vul())
        };
        return accepts(system.infer(vul, prefix).rho());
    }

    accepts(context.inferences().get(relative))
}

/// Require the opponent's declared system to show cards in the suit it named.
/// This keeps PEN's natural defenses off artificial strong-club/relay openings.
fn disclosed_natural_suit_opening(
    suit: Suit,
    context: &crate::bidding::context::Context<'_>,
) -> bool {
    opponent_opening_satisfies(context, |shown| shown.length(suit).min >= 3)
}

fn natural_suit_opening(suit: Suit) -> Cons<impl Constraint + Clone> {
    described(
        format!("the opponents disclose a natural {suit} opening"),
        move |_: Hand, context: &crate::bidding::context::Context<'_>| {
            disclosed_natural_suit_opening(suit, context)
        },
    )
}

/// Require a disclosed artificial strong-club opening.
fn disclosed_artificial_strong_club(context: &crate::bidding::context::Context<'_>) -> bool {
    opponent_opening_satisfies(context, |shown| {
        shown.strength.hcp.min >= 16 && shown.length(Suit::Clubs).min < 3
    })
}

fn artificial_strong_club() -> Cons<impl Constraint + Clone> {
    described(
        "the opponents disclose an artificial strong-club opening",
        |_: Hand, context: &crate::bidding::context::Context<'_>| {
            disclosed_artificial_strong_club(context)
        },
    )
}

fn takeout_or_strong(minimum: u8) -> Cons<impl Constraint + Clone> {
    (hcp(minimum..=16) & !balanced() & short_in_their_suits() & unbid_support(1)) | hcp(17..)
}

fn minor_takeout_or_strong(minimum: u8) -> Cons<impl Constraint + Clone> {
    (hcp(minimum..=16)
        & !balanced()
        & short_in_their_suits()
        & unbid_support(1)
        & and([Suit::Hearts, Suit::Spades], 3..=4)
        & or([Suit::Hearts, Suit::Spades], 4..=4))
        | hcp(17..)
}

/// Simple, Mathe-style interference over an artificial strong 1♣, with the
/// two two-suited calls swapped so the requested 1NT shows both majors.
fn strong_club_defense() -> Rules {
    let artificial = artificial_strong_club();
    let weak_or_strong = hcp(..=11) | hcp(16..);
    let mut rules = Rules::new()
        .rule(
            Call::Double,
            520,
            artificial.clone() & and([Suit::Clubs, Suit::Diamonds], 5..) & weak_or_strong.clone(),
        )
        .alert(STRONG_CLUB_MINORS)
        .rule(
            bid(1, Strain::Notrump),
            520,
            artificial.clone() & and([Suit::Hearts, Suit::Spades], 5..) & weak_or_strong,
        )
        .alert(STRONG_CLUB_MAJORS)
        .rule(
            bid(2, Strain::Clubs),
            470,
            artificial.clone() & len(Suit::Clubs, 6..) & hcp(5..),
        );

    for suit in [Suit::Diamonds, Suit::Hearts, Suit::Spades] {
        let strain = Strain::from(suit);
        rules = rules
            .rule(
                bid(2, strain),
                430,
                artificial.clone() & len(suit, 6..) & hcp(5..=9) & min_level_is(1, strain),
            )
            .rule(
                bid(1, strain),
                400 + i16::from(strain.is_major()) * 10,
                artificial.clone() & len(suit, 5..) & hcp(8..) & min_level_is(1, strain),
            );
    }

    // This is a complete policy for the immediate decision. Do not let the
    // generic natural floor reinterpret an artificial 1♣ when no action fits.
    rules.rule(Call::Pass, 0, artificial)
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
    let mut rules = if matches!(opened, Suit::Clubs | Suit::Diamonds) {
        Rules::new().rule(
            Call::Double,
            500,
            natural.clone() & minor_takeout_or_strong(12),
        )
    } else {
        Rules::new().rule(Call::Double, 500, natural.clone() & takeout_or_strong(12))
    }
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
                    & preempt_strength(suit)
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
    let rules = rules
        .rule(
            bid(2, Strain::Notrump),
            470,
            natural & len(low, 5..) & len(high, 5..) & (hcp(..=11) | hcp(16..)),
        )
        .alert(UNUSUAL);
    if opened == Suit::Clubs {
        rules.chain(strong_club_defense())
    } else {
        rules
    }
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

fn doubled_overcall_advances(opened: Suit, overcall: Bid) -> Rules {
    let Some(our) = overcall.strain.suit() else {
        return Rules::new();
    };
    let mut rules = overcall_advances(opened, overcall)
        .rule(
            Call::Redouble,
            700,
            hcp(10..) & len(our, 3..) & top_honors(our, 1..),
        )
        .alert(BUSINESS)
        .rule(Call::Pass, 100, len(our, 3..));
    for suit in Suit::ASC {
        if suit == opened || suit == our {
            continue;
        }
        for level in 1..=3 {
            rules = rules.rule(
                bid(level, Strain::from(suit)),
                290,
                hcp(..=7) & len(suit, 5..) & min_level_is(level, Strain::from(suit)),
            );
        }
    }
    rules
}

fn overcaller_runout(opened: Suit, overcall: Bid) -> Rules {
    let Some(our) = overcall.strain.suit() else {
        return Rules::new();
    };
    let mut rules = Rules::new();
    for suit in Suit::ASC {
        if suit == opened || suit == our {
            continue;
        }
        for level in 1..=3 {
            rules = rules.rule(
                bid(level, Strain::from(suit)),
                500,
                len(suit, 5..) & min_level_is(level, Strain::from(suit)),
            );
        }
    }
    rules.rule(Call::Pass, 100, len(our, 5..))
}

fn takeout_advances(opened: Suit) -> Rules {
    let natural = natural_suit_opening(opened);
    let mut rules = Rules::new()
        .rule(
            Call::Pass,
            600,
            natural.clone() & hcp(6..) & len(opened, 5..) & top_honors(opened, 2..),
        )
        .rule(
            bid(2, Strain::from(opened)),
            500,
            natural.clone() & hcp(13..),
        )
        .alert(ADVANCE_CUE)
        .rule(
            bid(3, Strain::Notrump),
            450,
            natural.clone() & hcp(13..) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(2, Strain::Notrump),
            380,
            natural.clone() & hcp(11..=12) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(1, Strain::Notrump),
            320,
            natural.clone() & hcp(6..=10) & balanced() & stopper_in(opened),
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
                    natural.clone()
                        & hcp(..=8)
                        & len(suit, 4..)
                        & min_level_is(level, Strain::from(suit)),
                )
                .rule(
                    bid(level + 1, Strain::from(suit)),
                    300,
                    natural.clone()
                        & hcp(9..=11)
                        & len(suit, 4..)
                        & min_level_is(level, Strain::from(suit)),
                );
        }
    }
    if opened == Suit::Clubs {
        let artificial = artificial_strong_club();
        rules = rules
            .rule(
                bid(2, Strain::Diamonds),
                550,
                artificial.clone() & longer_suit(Suit::Diamonds, Suit::Clubs),
            )
            .rule(
                bid(2, Strain::Clubs),
                540,
                artificial & at_least_as_long(Suit::Clubs, Suit::Diamonds),
            );
    }
    rules
}

fn balancing_defense(opened: Suit) -> Rules {
    let natural = natural_suit_opening(opened);
    let mut rules = Rules::new()
        .rule(Call::Double, 500, natural.clone() & takeout_or_strong(9))
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
        .rule(Call::Double, 500, natural.clone() & takeout_or_strong(12))
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
            .rule(Call::Double, 500, natural.clone() & takeout_or_strong(12))
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
    let natural = natural_suit_opening(opened);
    let mut rules = Rules::new()
        .rule(
            bid(3, Strain::Notrump),
            360,
            natural.clone() & hcp(18..) & balanced() & stopper_in(opened),
        )
        .rule(
            bid(2, Strain::from(response)),
            340,
            natural.clone()
                & hcp(18..)
                & len(response, 4..)
                & min_level_is(2, Strain::from(response)),
        );
    for suit in Suit::ASC {
        if suit == opened || suit == response {
            continue;
        }
        for level in 1..=3 {
            rules = rules.rule(
                bid(level, Strain::from(suit)),
                330,
                natural.clone()
                    & hcp(18..)
                    & len(suit, 5..)
                    & min_level_is(level, Strain::from(suit)),
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

fn michaels_advance(opened: Suit) -> Rules {
    if matches!(opened, Suit::Clubs | Suit::Diamonds) {
        return Rules::new()
            .rule(
                bid(2, Strain::Spades),
                300,
                longer_suit(Suit::Spades, Suit::Hearts),
            )
            .rule(
                bid(2, Strain::Hearts),
                290,
                at_least_as_long(Suit::Hearts, Suit::Spades),
            );
    }
    let other_major = if opened == Suit::Hearts {
        Suit::Spades
    } else {
        Suit::Hearts
    };
    let landing = if opened == Suit::Hearts { 2 } else { 3 };
    Rules::new()
        .rule(bid(2, Strain::Notrump), 400, hcp(10..))
        .alert(MICHAELS_ASK)
        .rule(bid(landing, Strain::from(other_major)), 100, hcp(0..))
}

fn unusual_advance(opened: Suit) -> Rules {
    let (low, high) = match opened {
        Suit::Clubs => (Suit::Diamonds, Suit::Hearts),
        Suit::Diamonds => (Suit::Clubs, Suit::Hearts),
        Suit::Hearts | Suit::Spades => (Suit::Clubs, Suit::Diamonds),
    };
    Rules::new()
        .rule(bid(3, Strain::from(high)), 300, longer_suit(high, low))
        .rule(bid(3, Strain::from(low)), 290, at_least_as_long(low, high))
}

fn strong_club_major_advance() -> Rules {
    let artificial = artificial_strong_club();
    Rules::new()
        .rule(
            bid(2, Strain::Spades),
            250,
            artificial.clone() & longer_suit(Suit::Spades, Suit::Hearts),
        )
        .rule(
            bid(2, Strain::Hearts),
            240,
            artificial & at_least_as_long(Suit::Hearts, Suit::Spades),
        )
}

fn strong_club_minor_signoff() -> Rules {
    Rules::new().rule(Call::Pass, 100, artificial_strong_club())
}

fn strong_club_major_signoff() -> Rules {
    Rules::new().rule(Call::Pass, 100, artificial_strong_club())
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
                if opened == Suit::Clubs && response == Suit::Diamonds {
                    doubler_rebid(opened, response).chain(strong_club_minor_signoff())
                } else {
                    doubler_rebid(opened, response)
                },
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
            entries.extend(rows_of(
                Pattern::node(&format!("{root} {overcall} (X)")),
                doubled_overcall_advances(opened, overcall),
            ));
            entries.extend(rows_of(
                Pattern::node(&format!("{root} {overcall} (X) P -")),
                overcaller_runout(opened, overcall),
            ));

            let jump = bid(level + 1, Strain::from(over));
            if jump.level.get() <= 3 {
                entries.extend(rows_of(
                    Pattern::node(&format!("{root} {jump} -")),
                    overcall_advances(opened, jump),
                ));
                entries.extend(rows_of(
                    Pattern::node(&format!("{root} {jump} (X)")),
                    doubled_overcall_advances(opened, jump),
                ));
                entries.extend(rows_of(
                    Pattern::node(&format!("{root} {jump} (X) P -")),
                    overcaller_runout(opened, jump),
                ));
            }
        }
        let cue = bid(2, Strain::from(opened));
        entries.extend(rows_of(
            Pattern::node(&format!("{root} {cue} (X)")),
            michaels_advance(opened),
        ));
        entries.extend(rows_of(
            Pattern::node(&format!("{root} 2NT (X)")),
            unusual_advance(opened),
        ));
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
            entries.extend(rows_of(
                Pattern::node(&format!(
                    "{root} {} (X) 2NT -",
                    bid(2, Strain::from(opened))
                )),
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
    entries.extend(rows_of(Pattern::node("P* (1NT) 2♣ (X)"), landy_advance()));
    entries.extend(rows_of(
        Pattern::node("P* (1♣) X - 2♣ -"),
        strong_club_minor_signoff(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* (1♣) 1NT -"),
        strong_club_major_advance(),
    ));
    for major in [Suit::Hearts, Suit::Spades] {
        entries.extend(rows_of(
            Pattern::node(&format!("P* (1♣) 1NT - {} -", bid(2, Strain::from(major)))),
            strong_club_major_signoff(),
        ));
    }
    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-defense",
        gate: |_| true,
        entries,
    }
}
