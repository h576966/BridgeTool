//! PEN-Club continuations after the 12–15 1NT opening.

use crate::bidding::agreements::Agreements;
use crate::bidding::constraint::{at_least_as_long, balanced, hcp, len, longer_suit, top_honors};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Strain, Suit};

pub(super) const MAJOR_TRANSFER: Alert = Alert("pen:major-transfer");
pub(super) const MINOR_TRANSFER: Alert = Alert("pen:minor-transfer");
pub(super) const TRANSFER_COMPLETION: Alert = Alert("pen:transfer-completion");
const STAYMAN: Alert = Alert("pen:stayman");
const STAYMAN_ANSWER: Alert = Alert("pen:stayman-answer");
const BOTH_MAJORS: Alert = Alert("pen:stayman-both-majors-signoff");
const TEXAS: Alert = Alert("pen:texas-transfer");
const QUANTITATIVE: Alert = Alert("pen:quantitative-four-notrump");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

pub(super) fn responses() -> Rules {
    Rules::new()
        .rule(
            bid(4, Strain::Diamonds),
            600,
            hcp(10..) & len(Suit::Hearts, 6..),
        )
        .alert(TEXAS)
        .rule(
            bid(4, Strain::Hearts),
            600,
            hcp(10..) & len(Suit::Spades, 6..),
        )
        .alert(TEXAS)
        .rule(
            bid(2, Strain::Diamonds),
            500,
            len(Suit::Hearts, 5..) & at_least_as_long(Suit::Hearts, Suit::Spades),
        )
        .alert(MAJOR_TRANSFER)
        .rule(
            bid(2, Strain::Hearts),
            500,
            len(Suit::Spades, 5..) & longer_suit(Suit::Spades, Suit::Hearts),
        )
        .alert(MAJOR_TRANSFER)
        .rule(bid(4, Strain::Notrump), 470, hcp(18..=19) & balanced())
        .alert(QUANTITATIVE)
        .rule(
            bid(3, Strain::Clubs),
            440,
            hcp(10..=12) & len(Suit::Clubs, 6..) & top_honors(Suit::Clubs, 2..),
        )
        .rule(
            bid(3, Strain::Diamonds),
            440,
            hcp(10..=12) & len(Suit::Diamonds, 6..) & top_honors(Suit::Diamonds, 2..),
        )
        .rule(
            bid(2, Strain::Spades),
            420,
            (hcp(..=9) | hcp(13..))
                & len(Suit::Clubs, 6..)
                & len(Suit::Diamonds, ..6)
                & len(Suit::Hearts, ..5)
                & len(Suit::Spades, ..5),
        )
        .alert(MINOR_TRANSFER)
        .rule(
            bid(2, Strain::Notrump),
            420,
            (hcp(..=9) | hcp(13..))
                & len(Suit::Diamonds, 6..)
                & len(Suit::Clubs, ..6)
                & len(Suit::Hearts, ..5)
                & len(Suit::Spades, ..5),
        )
        .alert(MINOR_TRANSFER)
        .rule(
            bid(2, Strain::Clubs),
            380,
            (hcp(10..) & (len(Suit::Hearts, 4..) | len(Suit::Spades, 4..)))
                | (hcp(..=9) & len(Suit::Hearts, 4..) & len(Suit::Spades, 4..))
                | (hcp(10..=12) & balanced()),
        )
        .alert(STAYMAN)
        .rule(bid(3, Strain::Notrump), 300, hcp(13..=17) & balanced())
        .rule(Call::Pass, 100, hcp(..=9))
}

fn stayman_answers() -> Rules {
    Rules::new()
        .rule(bid(2, Strain::Hearts), 300, len(Suit::Hearts, 4..))
        .alert(STAYMAN_ANSWER)
        .rule(
            bid(2, Strain::Spades),
            280,
            len(Suit::Hearts, 3..=3) & len(Suit::Spades, 4..),
        )
        .alert(STAYMAN_ANSWER)
        .rule(
            bid(2, Strain::Diamonds),
            200,
            len(Suit::Hearts, 3..=3) & len(Suit::Spades, 3..=3),
        )
        .alert(STAYMAN_ANSWER)
}

fn stayman_continuations(denial: bool) -> Rules {
    let mut rules = Rules::new()
        .rule(bid(3, Strain::Notrump), 300, hcp(13..))
        .rule(bid(2, Strain::Notrump), 200, hcp(10..=12));
    if denial {
        rules = rules
            .rule(
                bid(2, Strain::Hearts),
                400,
                hcp(..=9) & len(Suit::Hearts, 4..) & len(Suit::Spades, 4..),
            )
            .alert(BOTH_MAJORS);
    }
    rules
}

pub(super) fn major_transfer_accepts(major: Suit) -> Rules {
    Rules::new()
        .rule(
            bid(3, Strain::from(major)),
            400,
            hcp(15..=15) & len(major, 4..=4),
        )
        .alert(TRANSFER_COMPLETION)
        .rule(bid(2, Strain::from(major)), 100, hcp(0..))
        .alert(TRANSFER_COMPLETION)
}

fn transfer_continuations(major: Suit) -> Rules {
    let other = if major == Suit::Hearts {
        Suit::Spades
    } else {
        Suit::Hearts
    };
    let mut rules = Rules::new()
        .rule(
            bid(4, Strain::Notrump),
            500,
            hcp(18..=19) & len(major, 5..=5),
        )
        .alert(QUANTITATIVE)
        .rule(
            bid(4, Strain::from(major)),
            450,
            hcp(13..) & len(major, 6..),
        )
        .rule(
            bid(3, Strain::Notrump),
            430,
            hcp(13..=17) & len(major, 5..=5),
        )
        .rule(
            bid(3, Strain::from(major)),
            300,
            hcp(10..=12) & len(major, 6..),
        )
        .rule(
            bid(2, Strain::Notrump),
            280,
            hcp(10..=12) & len(major, 5..=5),
        );

    let other_level = if major == Suit::Hearts { 2 } else { 3 };
    rules = rules.rule(
        bid(other_level, Strain::from(other)),
        420,
        hcp(13..) & len(major, 5..) & len(other, 4..),
    );
    for minor in [Suit::Clubs, Suit::Diamonds] {
        rules = rules.rule(
            bid(3, Strain::from(minor)),
            410,
            hcp(13..) & len(major, 5..) & len(minor, 4..),
        );
    }
    rules.rule(Call::Pass, 100, hcp(..=9))
}

pub(super) fn complete_minor_transfer(minor: Suit) -> Rules {
    Rules::new()
        .rule(bid(3, Strain::from(minor)), 100, hcp(0..))
        .alert(TRANSFER_COMPLETION)
}

fn minor_transfer_continuations() -> Rules {
    Rules::new().rule(Call::Pass, 100, hcp(..=9))
}

pub(super) fn complete_texas(target: Suit) -> Rules {
    Rules::new()
        .rule(bid(4, Strain::from(target)), 100, hcp(0..))
        .alert(TRANSFER_COMPLETION)
}

fn quantitative_answer() -> Rules {
    Rules::new()
        .rule(bid(6, Strain::Notrump), 200, hcp(15..=15))
        .rule(Call::Pass, 100, hcp(12..=14))
}

fn texas_continuations(target: Suit) -> Rules {
    super::slam::direct_rkcb(target, 6).rule(Call::Pass, 100, hcp(..=17))
}

fn entries(_: &Agreements) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node("P* 1NT -"), responses());
    entries.extend(rows_of(Pattern::node("P* 1NT - 2♣ -"), stayman_answers()));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♣ - 2♦ -"),
        stayman_continuations(true),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♣ - 2♥ -"),
        stayman_continuations(false),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♣ - 2♠ -"),
        stayman_continuations(false),
    ));

    for (response, completion, major) in [("2♦", "2♥", Suit::Hearts), ("2♥", "2♠", Suit::Spades)]
    {
        let accept = if major == Suit::Hearts {
            Pattern::node("P* 1NT - 2♦ -")
        } else {
            Pattern::node("P* 1NT - 2♥ -")
        };
        entries.extend(rows_of(accept, major_transfer_accepts(major)));
        let continuation = if response == "2♦" {
            Pattern::node("P* 1NT - 2♦ - 2♥ -")
        } else {
            Pattern::node("P* 1NT - 2♥ - 2♠ -")
        };
        let _ = completion;
        entries.extend(rows_of(continuation, transfer_continuations(major)));
    }

    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♠ -"),
        complete_minor_transfer(Suit::Clubs),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♠ - 3♣ -"),
        minor_transfer_continuations(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2NT -"),
        complete_minor_transfer(Suit::Diamonds),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2NT - 3♦ -"),
        minor_transfer_continuations(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 4♦ -"),
        complete_texas(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 4♦ - 4♥ -"),
        texas_continuations(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 4♥ -"),
        complete_texas(Suit::Spades),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 4♥ - 4♠ -"),
        texas_continuations(Suit::Spades),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 4NT -"),
        quantitative_answer(),
    ));
    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-one-notrump",
        gate: |_| true,
        entries,
    }
}
