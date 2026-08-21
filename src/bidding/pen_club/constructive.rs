//! Uncontested PEN-Club continuations that are explicit in the source.

use crate::bidding::agreements::Agreements;
use crate::bidding::constraint::{
    Cons, Constraint, at_least_as_long, balanced, hcp, len, longer_suit, reads_as,
};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::{Bid, Strain, Suit};

use super::strength::{limited_maximum, limited_minimum};

const ONE_CLUB_NEGATIVE: Alert = Alert("pen:one-club-negative");
const ONE_CLUB_POSITIVE: Alert = Alert("pen:one-club-positive");
const ONE_CLUB_BALANCED_FORCE: Alert = Alert("pen:one-club-balanced-force");
const BOTH_MINORS: Alert = Alert("pen:both-minors");
const HEART_RELAY: Alert = Alert("pen:heart-relay");
const FORCING_SPADE: Alert = Alert("pen:forcing-spade-response");
const STENBERG: Alert = Alert("pen:stenberg");
const THREE_CARD_SUPPORT: Alert = Alert("pen:three-card-support");
const FORCING_NATURAL: Alert = Alert("pen:forcing-natural");
const SUPER_MARMIC: Alert = Alert("pen:super-marmic");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

fn marmic() -> Cons<impl Constraint + Clone> {
    (len(Suit::Clubs, 4..=4) & len(Suit::Diamonds, 4..=4) & len(Suit::Hearts, 4..=4))
        | (len(Suit::Clubs, 4..=4) & len(Suit::Diamonds, 4..=4) & len(Suit::Spades, 4..=4))
        | (len(Suit::Clubs, 4..=4) & len(Suit::Hearts, 4..=4) & len(Suit::Spades, 4..=4))
        | (len(Suit::Diamonds, 4..=4) & len(Suit::Hearts, 4..=4) & len(Suit::Spades, 4..=4))
}

fn positive_major_shape(major: Suit) -> Cons<impl Constraint + Clone> {
    len(major, 5..)
        | (len(major, 4..=4) & (marmic() | len(Suit::Clubs, 5..=5) | len(Suit::Diamonds, 5..=5)))
}

fn one_club_responses() -> Rules {
    // The source gives no tie-break for equal positive majors. The draft bids
    // hearts on equal length and records that choice as an open system question.
    Rules::new()
        .rule(
            bid(2, Strain::Hearts),
            400,
            hcp(5..=8) & len(Suit::Hearts, 6..),
        )
        .rule(
            bid(2, Strain::Spades),
            400,
            hcp(5..=8) & len(Suit::Spades, 6..),
        )
        .rule(
            bid(1, Strain::Hearts),
            320,
            hcp(9..)
                & positive_major_shape(Suit::Hearts)
                & at_least_as_long(Suit::Hearts, Suit::Spades),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(1, Strain::Spades),
            320,
            hcp(9..) & positive_major_shape(Suit::Spades) & longer_suit(Suit::Spades, Suit::Hearts),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(3, Strain::Clubs),
            300,
            hcp(9..) & len(Suit::Clubs, 5..) & len(Suit::Diamonds, 5..),
        )
        .alert(BOTH_MINORS)
        .rule(bid(2, Strain::Clubs), 280, hcp(9..) & len(Suit::Clubs, 6..))
        .alert(FORCING_NATURAL)
        .rule(
            bid(2, Strain::Diamonds),
            280,
            hcp(9..) & len(Suit::Diamonds, 6..),
        )
        .alert(FORCING_NATURAL)
        .rule(bid(2, Strain::Notrump), 240, hcp(12..=13) & balanced())
        .rule(
            bid(1, Strain::Notrump),
            220,
            (hcp(9..=11) | hcp(14..)) & balanced(),
        )
        .rule(bid(1, Strain::Diamonds), 100, hcp(..=8))
        .alert(ONE_CLUB_NEGATIVE)
}

fn one_club_negative_rebids() -> Rules {
    Rules::new()
        .rule(bid(2, Strain::Notrump), 500, hcp(25..) & balanced())
        .rule(
            bid(2, Strain::Hearts),
            450,
            hcp(19..=21) & len(Suit::Hearts, 6..),
        )
        .rule(
            bid(2, Strain::Spades),
            450,
            hcp(19..=21) & len(Suit::Spades, 6..),
        )
        .rule(
            bid(3, Strain::Clubs),
            430,
            hcp(16..=19) & len(Suit::Clubs, 5..) & len(Suit::Diamonds, 5..),
        )
        .alert(BOTH_MINORS)
        .rule(
            bid(2, Strain::Diamonds),
            420,
            hcp(16..=21) & len(Suit::Diamonds, 6..),
        )
        .rule(
            bid(1, Strain::Hearts),
            350,
            hcp(16..=21) & len(Suit::Hearts, 4..) & at_least_as_long(Suit::Hearts, Suit::Spades),
        )
        .rule(
            bid(1, Strain::Spades),
            350,
            hcp(16..=21) & len(Suit::Spades, 4..) & longer_suit(Suit::Spades, Suit::Hearts),
        )
        .rule(bid(2, Strain::Clubs), 320, hcp(19..=21) & balanced())
        .alert(ONE_CLUB_BALANCED_FORCE)
        .rule(
            bid(1, Strain::Notrump),
            300,
            hcp(16..=18) & balanced() & len(Suit::Hearts, ..5) & len(Suit::Spades, ..5),
        )
}

fn one_club_positive_major_rebids(major: Suit) -> Rules {
    let other = if major == Suit::Hearts {
        Suit::Spades
    } else {
        Suit::Hearts
    };
    let mut rules = Rules::new()
        .rule(bid(2, Strain::from(major)), 400, len(major, 3..))
        .rule(
            bid(3, Strain::Clubs),
            380,
            len(Suit::Clubs, 5..) & len(Suit::Diamonds, 5..),
        )
        .alert(BOTH_MINORS)
        .rule(bid(2, Strain::Clubs), 320, len(Suit::Clubs, 5..))
        .rule(bid(2, Strain::Diamonds), 320, len(Suit::Diamonds, 5..));
    if major == Suit::Hearts {
        rules = rules.rule(bid(1, Strain::Spades), 340, len(other, 4..));
    } else {
        rules = rules.rule(bid(2, Strain::Hearts), 340, len(other, 5..));
    }
    rules.rule(
        bid(1, Strain::Notrump),
        200,
        reads_as((balanced() | marmic()) & len(major, ..3), len(major, ..3)),
    )
}

fn one_club_supported_major(major: Suit) -> Rules {
    Rules::new()
        .rule(bid(2, Strain::Notrump), 200, len(major, 5..))
        .alert(STENBERG)
        .rule(
            bid(3, Strain::Clubs),
            180,
            len(major, 4..) & len(Suit::Clubs, 5..),
        )
        .rule(
            bid(3, Strain::Diamonds),
            180,
            len(major, 4..) & len(Suit::Diamonds, 5..),
        )
}

fn one_club_minor_rebids(minor: Suit) -> Rules {
    let mut rules = Rules::new()
        .rule(bid(3, Strain::from(minor)), 400, len(minor, 3..))
        .rule(
            bid(3, Strain::Notrump),
            380,
            hcp(16..=18) & balanced() & len(minor, 3..),
        )
        .rule(
            bid(2, Strain::Notrump),
            180,
            reads_as((balanced() | marmic()) & len(minor, ..3), len(minor, ..3)),
        );
    for suit in Suit::ASC {
        if suit == minor {
            continue;
        }
        let level = if minor == Suit::Diamonds && suit == Suit::Clubs {
            3
        } else {
            2
        };
        rules = rules.rule(bid(level, Strain::from(suit)), 250, len(suit, 5..));
    }
    rules
}

pub(super) fn one_diamond_responses() -> Rules {
    Rules::new()
        .rule(
            bid(3, Strain::Notrump),
            650,
            hcp(15..) & balanced() & len(Suit::Spades, 3..=3),
        )
        .alert(THREE_CARD_SUPPORT)
        .rule(
            bid(2, Strain::Hearts),
            400,
            hcp(..=8) & len(Suit::Hearts, 6..) & len(Suit::Spades, ..3),
        )
        .rule(
            bid(2, Strain::Clubs),
            330,
            hcp(15..) & len(Suit::Clubs, 4..),
        )
        .alert(FORCING_NATURAL)
        .rule(
            bid(2, Strain::Diamonds),
            330,
            hcp(15..) & len(Suit::Diamonds, 4..),
        )
        .alert(FORCING_NATURAL)
        .rule(
            bid(2, Strain::Spades),
            300,
            hcp(6..=9) & len(Suit::Spades, 4..),
        )
        .rule(bid(1, Strain::Hearts), 240, len(Suit::Hearts, 4..))
        .rule(bid(1, Strain::Spades), 230, len(Suit::Spades, 3..))
        .rule(
            bid(1, Strain::Notrump),
            160,
            hcp(..=13) & len(Suit::Hearts, ..4) & len(Suit::Spades, ..3),
        )
        .chain(super::slam::splinter_and_rkcb(
            Suit::Spades,
            &[Suit::Clubs, Suit::Diamonds, Suit::Hearts],
            3,
        ))
}

fn one_diamond_heart_rebid() -> Rules {
    Rules::new()
        .rule(bid(1, Strain::Notrump), 200, len(Suit::Hearts, 3..=3))
        .alert(THREE_CARD_SUPPORT)
}

fn one_diamond_spade_rebids() -> Rules {
    Rules::new()
        .rule(
            bid(1, Strain::Notrump),
            400,
            limited_maximum()
                & len(Suit::Spades, 5..=5)
                & len(Suit::Clubs, ..5)
                & len(Suit::Diamonds, ..5)
                & len(Suit::Hearts, ..5),
        )
        .alert(THREE_CARD_SUPPORT)
        .rule(
            bid(2, Strain::Clubs),
            300,
            len(Suit::Spades, 4..=4) & len(Suit::Clubs, 5..),
        )
        .rule(
            bid(2, Strain::Diamonds),
            300,
            len(Suit::Spades, 4..=4) & len(Suit::Diamonds, 5..),
        )
        .rule(
            bid(2, Strain::Hearts),
            300,
            len(Suit::Spades, 4..=4) & len(Suit::Hearts, 5..),
        )
}

fn one_heart_responses() -> Rules {
    Rules::new()
        .rule(
            bid(3, Strain::Notrump),
            650,
            hcp(15..) & balanced() & len(Suit::Hearts, 3..=3),
        )
        .alert(THREE_CARD_SUPPORT)
        .rule(
            bid(2, Strain::Notrump),
            500,
            hcp(11..) & len(Suit::Hearts, 4..),
        )
        .alert(STENBERG)
        .rule(
            bid(2, Strain::Spades),
            450,
            hcp(5..=8) & len(Suit::Spades, 6..),
        )
        .rule(bid(1, Strain::Notrump), 420, len(Suit::Spades, 5..))
        .alert(FORCING_SPADE)
        .rule(
            bid(2, Strain::Clubs),
            380,
            hcp(15..) & len(Suit::Clubs, 4..),
        )
        .alert(FORCING_NATURAL)
        .rule(
            bid(2, Strain::Diamonds),
            380,
            hcp(15..) & len(Suit::Diamonds, 4..),
        )
        .alert(FORCING_NATURAL)
        .rule(
            bid(3, Strain::Clubs),
            360,
            hcp(10..=11) & len(Suit::Clubs, 6..),
        )
        .rule(
            bid(3, Strain::Diamonds),
            360,
            hcp(10..=11) & len(Suit::Diamonds, 6..),
        )
        .rule(
            bid(3, Strain::Hearts),
            340,
            hcp(7..=10) & len(Suit::Hearts, 4..),
        )
        .rule(
            bid(2, Strain::Hearts),
            320,
            hcp(10..=12) & len(Suit::Hearts, 3..=3),
        )
        .rule(
            bid(1, Strain::Spades),
            100,
            hcp(..=10) & len(Suit::Spades, ..5),
        )
        .alert(HEART_RELAY)
        .chain(super::slam::splinter_and_rkcb(
            Suit::Hearts,
            &[Suit::Clubs, Suit::Diamonds],
            3,
        ))
}

fn heart_relay_rebids() -> Rules {
    Rules::new()
        .rule(
            bid(3, Strain::Spades),
            500,
            limited_maximum()
                & len(Suit::Spades, 0..=0)
                & len(Suit::Hearts, 4..)
                & len(Suit::Clubs, 4..)
                & len(Suit::Diamonds, 4..),
        )
        .alert(SUPER_MARMIC)
        .rule(
            bid(3, Strain::Hearts),
            480,
            limited_maximum() & len(Suit::Hearts, 7..),
        )
        .rule(
            bid(3, Strain::Clubs),
            460,
            limited_maximum() & len(Suit::Clubs, 6..) & len(Suit::Hearts, 4..),
        )
        .rule(
            bid(3, Strain::Diamonds),
            460,
            limited_maximum() & len(Suit::Diamonds, 6..) & len(Suit::Hearts, 4..),
        )
        .rule(
            bid(2, Strain::Spades),
            440,
            limited_maximum() & len(Suit::Hearts, 5..) & len(Suit::Clubs, 5..),
        )
        .alert(BOTH_MINORS)
        .rule(
            bid(2, Strain::Notrump),
            440,
            limited_maximum() & len(Suit::Hearts, 5..) & len(Suit::Diamonds, 5..),
        )
        .alert(BOTH_MINORS)
        .rule(
            bid(2, Strain::Hearts),
            420,
            limited_maximum() & len(Suit::Hearts, 6..),
        )
        .rule(
            bid(2, Strain::Clubs),
            300,
            len(Suit::Clubs, 5..) & len(Suit::Hearts, 4..),
        )
        .rule(
            bid(2, Strain::Diamonds),
            300,
            len(Suit::Diamonds, 5..) & len(Suit::Hearts, 4..),
        )
        .rule(bid(1, Strain::Notrump), 100, len(Suit::Hearts, 4..))
}

fn forcing_spade_rebids() -> Rules {
    Rules::new()
        .rule(
            bid(3, Strain::Spades),
            500,
            limited_maximum() & len(Suit::Spades, 3..) & len(Suit::Hearts, 4..=5),
        )
        .rule(
            bid(3, Strain::Hearts),
            480,
            len(Suit::Hearts, 6..) & len(Suit::Spades, 3..),
        )
        .rule(
            bid(3, Strain::Clubs),
            460,
            limited_maximum()
                & len(Suit::Clubs, 6..)
                & len(Suit::Hearts, 4..)
                & len(Suit::Spades, ..3),
        )
        .rule(
            bid(3, Strain::Diamonds),
            460,
            limited_maximum()
                & len(Suit::Diamonds, 6..)
                & len(Suit::Hearts, 4..)
                & len(Suit::Spades, ..3),
        )
        .rule(
            bid(2, Strain::Notrump),
            440,
            limited_maximum() & len(Suit::Hearts, 6..) & len(Suit::Spades, ..3),
        )
        .rule(
            bid(2, Strain::Spades),
            420,
            limited_minimum(10) & len(Suit::Spades, 3..),
        )
        .rule(
            bid(2, Strain::Clubs),
            300,
            len(Suit::Clubs, 5..) & len(Suit::Hearts, 4..),
        )
        .rule(
            bid(2, Strain::Diamonds),
            300,
            len(Suit::Diamonds, 5..) & len(Suit::Hearts, 4..),
        )
        .rule(bid(2, Strain::Hearts), 100, len(Suit::Hearts, 5..))
}

fn two_diamond_responses() -> Rules {
    Rules::new()
        .rule(
            bid(2, Strain::Notrump),
            400,
            hcp(10..) & len(Suit::Diamonds, 3..),
        )
        .alert(STENBERG)
        .rule(
            bid(3, Strain::Clubs),
            350,
            hcp(14..) & len(Suit::Clubs, 5..),
        )
        .alert(FORCING_NATURAL)
        .rule(
            bid(3, Strain::Diamonds),
            320,
            hcp(..=9) & len(Suit::Diamonds, 3..),
        )
        .rule(bid(2, Strain::Hearts), 420, len(Suit::Hearts, 5..))
        .alert(FORCING_NATURAL)
        .rule(bid(2, Strain::Spades), 420, len(Suit::Spades, 5..))
        .alert(FORCING_NATURAL)
}

fn two_diamond_major_rebids(major: Suit) -> Rules {
    let max_support = if major == Suit::Hearts {
        bid(2, Strain::Spades)
    } else {
        bid(3, Strain::Hearts)
    };
    Rules::new()
        .rule(max_support, 300, limited_maximum() & len(major, 3..))
        .alert(STENBERG)
        .rule(
            bid(2, Strain::Notrump),
            250,
            limited_maximum() & len(major, ..3),
        )
        .rule(
            bid(3, Strain::from(major)),
            200,
            limited_minimum(11) & len(major, 3..),
        )
}

fn entries(_: &Agreements) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node("P* 1♣ -"), one_club_responses());
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 1♦ -"),
        one_club_negative_rebids(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 1♥ -"),
        one_club_positive_major_rebids(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 1♠ -"),
        one_club_positive_major_rebids(Suit::Spades),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 1♥ - 2♥ -"),
        one_club_supported_major(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 1♠ - 2♠ -"),
        one_club_supported_major(Suit::Spades),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 2♣ -"),
        one_club_minor_rebids(Suit::Clubs),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 2♦ -"),
        one_club_minor_rebids(Suit::Diamonds),
    ));

    entries.extend(rows_of(Pattern::node("P* 1♦ -"), one_diamond_responses()));
    entries.extend(rows_of(
        Pattern::node("P* 1♦ - 1♥ -"),
        one_diamond_heart_rebid(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♦ - 1♠ -"),
        one_diamond_spade_rebids(),
    ));

    entries.extend(rows_of(Pattern::node("P* 1♥ -"), one_heart_responses()));
    entries.extend(rows_of(Pattern::node("P* 1♥ - 1♠ -"), heart_relay_rebids()));
    entries.extend(rows_of(
        Pattern::node("P* 1♥ - 1NT -"),
        forcing_spade_rebids(),
    ));

    entries.extend(rows_of(Pattern::node("P* 2♦ -"), two_diamond_responses()));
    entries.extend(rows_of(
        Pattern::node("P* 2♦ - 2♥ -"),
        two_diamond_major_rebids(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 2♦ - 2♠ -"),
        two_diamond_major_rebids(Suit::Spades),
    ));
    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-constructive",
        gate: |_| true,
        entries,
    }
}
