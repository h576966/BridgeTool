//! Explicit competitive tails for PEN-Club's artificial calls.

use super::constructive::{
    heart_relay_rebids, one_club_minor_rebids, one_club_negative_rebids,
    one_club_positive_major_rebids, one_club_responses, one_diamond_heart_rebid,
    one_diamond_responses, one_heart_responses,
};
use super::one_notrump::{
    complete_minor_transfer, complete_texas, major_transfer_accepts, responses as nt_responses,
};
use super::one_spade::{
    complete_transfer, general_ask_answers, redouble_ask_answers, responses_after_double,
    responses_over_one_notrump, responses_over_two_clubs, responses_over_two_diamonds,
    responses_over_two_hearts, shape_ask_answers,
};
use crate::bidding::agreements::Agreements;
use crate::bidding::american::NotrumpDefense;
use crate::bidding::constraint::{
    at_least_as_long, balanced, hcp, len, longer_suit, stopper_in, support_points, top_honors,
};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Strain, Suit};

const TWO_DIAMOND_TRANSFER: Alert = Alert("pen:two-diamond-doubled-transfer");
const TRANSFER_COMPLETION: Alert = Alert("pen:transfer-completion");
const RUBENSOHL: Alert = Alert("pen:one-spade-rubensohl");
const GENERAL_ASK: Alert = Alert("pen:one-spade-general-ask");
const BUSINESS: Alert = Alert("pen:business-redouble");
const ONE_CLUB_NEGATIVE_TRANSFER: Alert = Alert("pen:one-club-negative-transfer");
const ONE_CLUB_INTERFERENCE_FORCE: Alert = Alert("pen:one-club-interference-force");
const ONE_CLUB_POSITIVE: Alert = Alert("pen:one-club-interference-positive");
const ONE_CLUB_REOPENING_DOUBLE: Alert = Alert("pen:one-club-reopening-double");
const ONE_CLUB_STRONG_RELAY: Alert = Alert("pen:one-club-strong-relay");
const ONE_CLUB_TRANSFER_COMPLETION: Alert = Alert("pen:one-club-transfer-completion");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

fn two_diamond_doubled_responses() -> Rules {
    Rules::new()
        .rule(Call::Redouble, 300, len(Suit::Hearts, 5..))
        .alert(TWO_DIAMOND_TRANSFER)
        .rule(bid(2, Strain::Hearts), 300, len(Suit::Spades, 5..))
        .alert(TWO_DIAMOND_TRANSFER)
}

fn forced(call: Bid) -> Rules {
    Rules::new()
        .rule(call, 100, hcp(0..))
        .alert(TRANSFER_COMPLETION)
}

fn with_business_redouble(rules: Rules, suit: Suit) -> Rules {
    rules
        .rule(
            Call::Redouble,
            700,
            hcp(10..) & len(suit, 5..) & top_honors(suit, 2..),
        )
        .alert(BUSINESS)
}

fn doubled_one_heart_responses() -> Rules {
    one_heart_responses()
        .rule(Call::Redouble, 700, hcp(10..) & len(Suit::Hearts, 3..))
        .alert(BUSINESS)
}

fn one_diamond_heart_after_double() -> Rules {
    one_diamond_heart_rebid().rule(bid(1, Strain::Spades), 100, len(Suit::Spades, 4..))
}

fn doubled_natural_opening_responses(opened: Suit, level: u8) -> Rules {
    let mut rules = Rules::new()
        .rule(
            Call::Redouble,
            700,
            hcp(10..) & len(opened, 3..) & top_honors(opened, 1..),
        )
        .alert(BUSINESS);
    for suit in Suit::ASC {
        if suit == opened {
            continue;
        }
        let strain = Strain::from(suit);
        let landing = if strain > Strain::from(opened) {
            level
        } else {
            level + 1
        };
        rules = rules.rule(bid(landing, strain), 500, len(suit, 5..));
    }
    rules
        .rule(bid(level + 1, Strain::from(opened)), 350, len(opened, 3..))
        .rule(Call::Pass, 100, hcp(0..))
}

fn doubled_natural_opener_rebid(opened: Suit, level: u8) -> Rules {
    let mut rules = Rules::new();
    for suit in Suit::ASC {
        if suit == opened {
            continue;
        }
        let strain = Strain::from(suit);
        let landing = if strain > Strain::from(opened) {
            level
        } else {
            level + 1
        };
        rules = rules.rule(bid(landing, strain), 500, len(suit, 5..));
    }
    rules.rule(Call::Pass, 100, hcp(0..))
}

fn one_club_over_one_diamond() -> Rules {
    Rules::new()
        .rule(
            Call::Double,
            600,
            hcp(..=8)
                & len(Suit::Hearts, 5..)
                & at_least_as_long(Suit::Hearts, Suit::Spades)
                & at_least_as_long(Suit::Hearts, Suit::Clubs),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(
            bid(1, Strain::Hearts),
            600,
            hcp(..=8)
                & len(Suit::Spades, 5..)
                & longer_suit(Suit::Spades, Suit::Hearts)
                & at_least_as_long(Suit::Spades, Suit::Clubs),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(
            bid(1, Strain::Spades),
            600,
            hcp(..=8)
                & len(Suit::Clubs, 5..)
                & longer_suit(Suit::Clubs, Suit::Hearts)
                & longer_suit(Suit::Clubs, Suit::Spades),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(bid(2, Strain::Clubs), 500, hcp(9..) & len(Suit::Clubs, 6..))
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(2, Strain::Hearts),
            500,
            hcp(9..) & len(Suit::Hearts, 5..),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(2, Strain::Spades),
            500,
            hcp(9..) & len(Suit::Spades, 5..),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(1, Strain::Notrump),
            450,
            (hcp(9..=11) | hcp(14..)) & balanced() & stopper_in(Suit::Diamonds),
        )
        .rule(
            bid(2, Strain::Notrump),
            450,
            hcp(12..=13) & balanced() & stopper_in(Suit::Diamonds),
        )
        .rule(Call::Pass, 400, hcp(9..) & len(Suit::Diamonds, 5..))
        .rule(bid(2, Strain::Diamonds), 300, hcp(9..))
        .alert(ONE_CLUB_INTERFERENCE_FORCE)
        .rule(Call::Pass, 100, hcp(..=8))
}

fn one_club_over_one_heart() -> Rules {
    Rules::new()
        .rule(
            Call::Double,
            600,
            hcp(..=8)
                & len(Suit::Spades, 5..)
                & at_least_as_long(Suit::Spades, Suit::Clubs)
                & at_least_as_long(Suit::Spades, Suit::Diamonds),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(
            bid(1, Strain::Spades),
            600,
            hcp(..=8)
                & len(Suit::Clubs, 5..)
                & longer_suit(Suit::Clubs, Suit::Spades)
                & at_least_as_long(Suit::Clubs, Suit::Diamonds),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(
            bid(1, Strain::Notrump),
            600,
            hcp(..=8)
                & len(Suit::Diamonds, 5..)
                & longer_suit(Suit::Diamonds, Suit::Spades)
                & longer_suit(Suit::Diamonds, Suit::Clubs),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(bid(2, Strain::Clubs), 500, hcp(9..) & len(Suit::Clubs, 6..))
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(2, Strain::Diamonds),
            500,
            hcp(9..) & len(Suit::Diamonds, 6..),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(2, Strain::Spades),
            500,
            hcp(9..) & len(Suit::Spades, 5..),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(2, Strain::Notrump),
            450,
            hcp(9..) & balanced() & stopper_in(Suit::Hearts),
        )
        .rule(Call::Pass, 400, hcp(9..) & len(Suit::Hearts, 5..))
        .rule(bid(2, Strain::Hearts), 300, hcp(9..))
        .alert(ONE_CLUB_INTERFERENCE_FORCE)
        .rule(Call::Pass, 100, hcp(..=8))
}

fn one_club_over_one_spade() -> Rules {
    Rules::new()
        .rule(
            Call::Double,
            600,
            hcp(..=8)
                & len(Suit::Hearts, 5..)
                & at_least_as_long(Suit::Hearts, Suit::Clubs)
                & at_least_as_long(Suit::Hearts, Suit::Diamonds),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(
            bid(1, Strain::Notrump),
            600,
            hcp(..=8)
                & len(Suit::Clubs, 5..)
                & longer_suit(Suit::Clubs, Suit::Hearts)
                & at_least_as_long(Suit::Clubs, Suit::Diamonds),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(
            bid(2, Strain::Clubs),
            600,
            hcp(..=8)
                & len(Suit::Diamonds, 5..)
                & longer_suit(Suit::Diamonds, Suit::Hearts)
                & longer_suit(Suit::Diamonds, Suit::Clubs),
        )
        .alert(ONE_CLUB_NEGATIVE_TRANSFER)
        .rule(
            bid(2, Strain::Diamonds),
            500,
            hcp(9..) & len(Suit::Diamonds, 6..),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(2, Strain::Hearts),
            500,
            hcp(9..) & len(Suit::Hearts, 5..),
        )
        .alert(ONE_CLUB_POSITIVE)
        .rule(bid(3, Strain::Clubs), 500, hcp(9..) & len(Suit::Clubs, 6..))
        .alert(ONE_CLUB_POSITIVE)
        .rule(
            bid(2, Strain::Notrump),
            450,
            hcp(9..) & balanced() & stopper_in(Suit::Spades),
        )
        .rule(Call::Pass, 400, hcp(9..) & len(Suit::Spades, 5..))
        .rule(bid(2, Strain::Spades), 300, hcp(9..))
        .alert(ONE_CLUB_INTERFERENCE_FORCE)
        .rule(Call::Pass, 100, hcp(..=8))
}

fn one_club_reopening(over: Suit) -> Rules {
    let over_strain = Strain::from(over);
    let mut rules = Rules::new()
        .rule(bid(2, Strain::Clubs), 700, hcp(20..))
        .alert(ONE_CLUB_STRONG_RELAY);

    for suit in Suit::ASC {
        if suit == Suit::Clubs || suit == over {
            continue;
        }
        let strain = Strain::from(suit);
        let level = if strain > over_strain { 1 } else { 2 };
        rules = rules.rule(bid(level, strain), 550, hcp(16..=19) & len(suit, 5..));
    }

    rules
        .rule(
            bid(1, Strain::Notrump),
            500,
            hcp(16..=19) & balanced() & stopper_in(over),
        )
        .rule(Call::Double, 100, hcp(..=19))
        .alert(ONE_CLUB_REOPENING_DOUBLE)
}

fn one_club_transfer_rebids(over: Suit, shown: Suit, last_bid: Bid) -> Rules {
    let completion_level = if Strain::from(shown) > last_bid.strain {
        last_bid.level.get()
    } else {
        last_bid.level.get() + 1
    };
    let completion = bid(completion_level, Strain::from(shown));
    let mut rules = Rules::new();

    let strong_relay = bid(2, Strain::Clubs);
    if strong_relay > last_bid && strong_relay != completion {
        rules = rules
            .rule(strong_relay, 800, hcp(20..))
            .alert(ONE_CLUB_STRONG_RELAY);
    }

    for suit in Suit::ASC {
        if suit == Suit::Clubs || suit == over || suit == shown {
            continue;
        }
        let strain = Strain::from(suit);
        let level = if strain > last_bid.strain {
            last_bid.level.get()
        } else {
            last_bid.level.get() + 1
        };
        rules = rules
            .rule(bid(level, strain), 700, hcp(16..) & len(suit, 5..))
            .rule(bid(level, strain), 450, hcp(16..) & len(suit, 4..));
    }

    let nt_level = if Strain::Notrump > last_bid.strain {
        last_bid.level.get()
    } else {
        last_bid.level.get() + 1
    };
    rules
        .rule(completion, 650, hcp(16..) & len(shown, 3..))
        .alert(ONE_CLUB_TRANSFER_COMPLETION)
        .rule(
            bid(nt_level, Strain::Notrump),
            600,
            hcp(16..) & balanced() & stopper_in(over),
        )
        .rule(completion, 100, hcp(0..))
        .alert(ONE_CLUB_TRANSFER_COMPLETION)
}

fn natural_major_overcall(over: Suit) -> Rules {
    let other = if over == Suit::Hearts {
        Suit::Spades
    } else {
        Suit::Hearts
    };
    let level = if over == Suit::Hearts { 2 } else { 3 };
    Rules::new()
        .rule(Call::Double, 300, hcp(8..) & len(other, 3..))
        .rule(
            bid(level, Strain::from(other)),
            200,
            hcp(8..) & len(other, 5..),
        )
}

fn natural_minor_overcall(over: Suit) -> Rules {
    let other_minor = if over == Suit::Clubs {
        Suit::Diamonds
    } else {
        Suit::Clubs
    };
    Rules::new()
        .rule(
            Call::Double,
            300,
            hcp(8..) & len(Suit::Hearts, 4..) & len(Suit::Spades, 4..),
        )
        .rule(
            bid(2, Strain::Hearts),
            220,
            hcp(8..) & len(Suit::Hearts, 5..),
        )
        .rule(
            bid(2, Strain::Spades),
            220,
            hcp(8..) & len(Suit::Spades, 5..),
        )
        .rule(
            bid(3, Strain::from(other_minor)),
            200,
            hcp(8..) & len(other_minor, 5..),
        )
}

fn doubled_notrump_responses() -> Rules {
    nt_responses()
        .rule(Call::Redouble, 700, hcp(10..))
        .alert(BUSINESS)
}

fn rubensohl_responses() -> Rules {
    Rules::new()
        .rule(Call::Double, 600, hcp(12..))
        .alert(GENERAL_ASK)
        .rule(
            bid(3, Strain::Spades),
            550,
            hcp(13..) & stopper_in(Suit::Spades),
        )
        .alert(RUBENSOHL)
        .rule(
            bid(3, Strain::Notrump),
            530,
            hcp(13..) & !stopper_in(Suit::Spades),
        )
        .rule(
            bid(3, Strain::Hearts),
            500,
            hcp(12..=14) & len(Suit::Hearts, 6..),
        )
        .rule(
            bid(2, Strain::Notrump),
            400,
            (hcp(..=11) | hcp(15..)) & len(Suit::Clubs, 6..),
        )
        .alert(RUBENSOHL)
        .rule(
            bid(3, Strain::Clubs),
            400,
            (hcp(..=11) | hcp(15..)) & len(Suit::Diamonds, 6..),
        )
        .alert(RUBENSOHL)
        .rule(
            bid(3, Strain::Diamonds),
            400,
            (hcp(..=11) | hcp(15..)) & len(Suit::Hearts, 5..),
        )
        .alert(RUBENSOHL)
        .rule(Call::Pass, 100, hcp(..=11))
}

fn after_limited_diamond_answer() -> Rules {
    Rules::new()
        .rule(bid(3, Strain::Clubs), 400, hcp(15..))
        .alert(GENERAL_ASK)
        .rule(
            bid(3, Strain::Diamonds),
            300,
            hcp(12..=14) & len(Suit::Diamonds, 5..),
        )
        .rule(Call::Pass, 100, hcp(12..=14))
}

fn after_limited_club_answer() -> Rules {
    Rules::new()
        .rule(bid(3, Strain::Diamonds), 400, hcp(15..))
        .alert(GENERAL_ASK)
        .rule(Call::Pass, 100, hcp(12..=14))
}

fn rubensohl_after_completion(minor: Option<Suit>, relay: Bid) -> Rules {
    let mut rules = Rules::new().rule(Call::Pass, 100, hcp(..=11));
    rules = rules.rule(relay, 400, hcp(15..)).alert(GENERAL_ASK);
    if let Some(minor) = minor {
        rules = rules.rule(
            bid(4, Strain::from(minor)),
            300,
            hcp(12..=14) & len(minor, 6..),
        );
    }
    rules
}

fn direct_three_notrump_answer() -> Rules {
    Rules::new()
        .rule(Call::Pass, 500, stopper_in(Suit::Spades))
        .rule(
            bid(4, Strain::Hearts),
            400,
            !stopper_in(Suit::Spades) & len(Suit::Hearts, 4..),
        )
        .rule(
            bid(4, Strain::Diamonds),
            300,
            !stopper_in(Suit::Spades)
                & len(Suit::Diamonds, 5..)
                & at_least_as_long(Suit::Diamonds, Suit::Clubs),
        )
        .rule(
            bid(4, Strain::Clubs),
            300,
            !stopper_in(Suit::Spades) & longer_suit(Suit::Clubs, Suit::Diamonds),
        )
}

fn entries(agreements: &Agreements) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node("P* 1♣ (1♦)"), one_club_over_one_diamond());
    entries.extend(rows_of(
        Pattern::node("P* 1♣ (1♥)"),
        one_club_over_one_heart(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ (1♠)"),
        one_club_over_one_spade(),
    ));

    for (pattern, over) in [
        ("P* 1♣ (1♦) P -", Suit::Diamonds),
        ("P* 1♣ (1♥) P -", Suit::Hearts),
        ("P* 1♣ (1♠) P -", Suit::Spades),
    ] {
        entries.extend(rows_of(Pattern::node(pattern), one_club_reopening(over)));
    }

    for (pattern, over, shown, last_bid) in [
        (
            "P* 1♣ (1♦) X -",
            Suit::Diamonds,
            Suit::Hearts,
            bid(1, Strain::Diamonds),
        ),
        (
            "P* 1♣ (1♦) 1♥ -",
            Suit::Diamonds,
            Suit::Spades,
            bid(1, Strain::Hearts),
        ),
        (
            "P* 1♣ (1♦) 1♠ -",
            Suit::Diamonds,
            Suit::Clubs,
            bid(1, Strain::Spades),
        ),
        (
            "P* 1♣ (1♥) X -",
            Suit::Hearts,
            Suit::Spades,
            bid(1, Strain::Hearts),
        ),
        (
            "P* 1♣ (1♥) 1♠ -",
            Suit::Hearts,
            Suit::Clubs,
            bid(1, Strain::Spades),
        ),
        (
            "P* 1♣ (1♥) 1NT -",
            Suit::Hearts,
            Suit::Diamonds,
            bid(1, Strain::Notrump),
        ),
        (
            "P* 1♣ (1♠) X -",
            Suit::Spades,
            Suit::Hearts,
            bid(1, Strain::Spades),
        ),
        (
            "P* 1♣ (1♠) 1NT -",
            Suit::Spades,
            Suit::Clubs,
            bid(1, Strain::Notrump),
        ),
        (
            "P* 1♣ (1♠) 2♣ -",
            Suit::Spades,
            Suit::Diamonds,
            bid(2, Strain::Clubs),
        ),
    ] {
        entries.extend(rows_of(
            Pattern::node(pattern),
            one_club_transfer_rebids(over, shown, last_bid),
        ));
    }

    entries.extend(rows_of(
        Pattern::node("P* 1♣ (X)"),
        with_business_redouble(one_club_responses(), Suit::Clubs),
    ));
    for pattern in ["P* 1♣ (X) 1♦ -", "P* 1♣ - 1♦ (X)"] {
        entries.extend(rows_of(Pattern::node(pattern), one_club_negative_rebids()));
    }
    for (response, major) in [("1♥", Suit::Hearts), ("1♠", Suit::Spades)] {
        for pattern in [
            format!("P* 1♣ (X) {response} -"),
            format!("P* 1♣ - {response} (X)"),
        ] {
            entries.extend(rows_of(
                Pattern::node(&pattern),
                one_club_positive_major_rebids(major),
            ));
        }
    }
    for (response, minor) in [("2♣", Suit::Clubs), ("2♦", Suit::Diamonds)] {
        for pattern in [
            format!("P* 1♣ (X) {response} -"),
            format!("P* 1♣ - {response} (X)"),
        ] {
            entries.extend(rows_of(
                Pattern::node(&pattern),
                one_club_minor_rebids(minor),
            ));
        }
    }

    entries.extend(rows_of(
        Pattern::node("P* 1♦ (X)"),
        with_business_redouble(one_diamond_responses(), Suit::Diamonds),
    ));
    for pattern in ["P* 1♦ (X) 1♥ -", "P* 1♦ - 1♥ (X)"] {
        entries.extend(rows_of(
            Pattern::node(pattern),
            one_diamond_heart_after_double(),
        ));
    }
    entries.extend(rows_of(
        Pattern::node("P* 1♥ (X)"),
        doubled_one_heart_responses(),
    ));
    for pattern in ["P* 1♥ (X) 1♠ -", "P* 1♥ - 1♠ (X)"] {
        entries.extend(rows_of(Pattern::node(pattern), heart_relay_rebids()));
    }

    for (opening, suit) in [
        ("2♣", Suit::Clubs),
        ("2♥", Suit::Hearts),
        ("2♠", Suit::Spades),
    ] {
        entries.extend(rows_of(
            Pattern::node(&format!("P* {opening} (X)")),
            doubled_natural_opening_responses(suit, 2),
        ));
        entries.extend(rows_of(
            Pattern::node(&format!("P* {opening} (X) P -")),
            doubled_natural_opener_rebid(suit, 2),
        ));
    }
    entries.extend(rows_of(
        Pattern::node("P* 2♦ (X) P -"),
        doubled_natural_opener_rebid(Suit::Diamonds, 2),
    ));

    entries.extend(rows_of(
        Pattern::node("P* 1♠ (X)"),
        responses_after_double(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (X) XX -"),
        redouble_ask_answers(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (1NT)"),
        responses_over_one_notrump(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♣)"),
        responses_over_two_clubs(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♦)"),
        responses_over_two_diamonds(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♥)"),
        responses_over_two_hearts(),
    ));
    entries.extend(rows_of(Pattern::node("P* 1♠ (2♠)"), rubensohl_responses()));

    for pattern in ["P* 1♠ (1NT) 2♠ -", "P* 1♠ (2♣) 2♠ -", "P* 1♠ (2♠) X -"] {
        entries.extend(rows_of(Pattern::node(pattern), general_ask_answers()));
    }
    for pattern in ["P* 1♠ (1NT) 2NT -", "P* 1♠ (2♣) 2NT -"] {
        entries.extend(rows_of(Pattern::node(pattern), shape_ask_answers()));
    }

    for pattern in [
        "P* 1♠ - 2♦ (X)",
        "P* 1♠ (X) 2♦ -",
        "P* 1♠ (1NT) 2♦ -",
        "P* 1♠ (2♣) 2♦ -",
        "P* 1♠ (2♦) X -",
    ] {
        entries.extend(rows_of(
            Pattern::node(pattern),
            complete_transfer(Suit::Hearts),
        ));
    }
    for pattern in [
        "P* 1♠ - 2♥ (X)",
        "P* 1♠ (X) 2♥ -",
        "P* 1♠ (1NT) 2♥ -",
        "P* 1♠ (2♣) 2♥ -",
        "P* 1♠ (2♦) 2♥ -",
        "P* 1♠ (2♥) X -",
    ] {
        entries.extend(rows_of(
            Pattern::node(pattern),
            complete_transfer(Suit::Spades),
        ));
    }

    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♠) X - 2NT -"),
        after_limited_diamond_answer(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♠) X - 3♣ -"),
        after_limited_club_answer(),
    ));

    for (pattern, completion) in [
        ("P* 1♠ (2♠) 2NT -", bid(3, Strain::Clubs)),
        ("P* 1♠ (2♠) 3♣ -", bid(3, Strain::Diamonds)),
        ("P* 1♠ (2♠) 3♦ -", bid(3, Strain::Hearts)),
        ("P* 1♠ (2♠) 3♠ -", bid(3, Strain::Notrump)),
        ("P* 1♠ (2♠) 2NT (X)", bid(3, Strain::Clubs)),
        ("P* 1♠ (2♠) 3♣ (X)", bid(3, Strain::Diamonds)),
        ("P* 1♠ (2♠) 3♦ (X)", bid(3, Strain::Hearts)),
    ] {
        entries.extend(rows_of(Pattern::node(pattern), forced(completion)));
    }
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♠) 2NT - 3♣ -"),
        rubensohl_after_completion(Some(Suit::Clubs), bid(3, Strain::Diamonds)),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♠) 3♣ - 3♦ -"),
        rubensohl_after_completion(Some(Suit::Diamonds), bid(3, Strain::Hearts)),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♠) 3♦ - 3♥ -"),
        rubensohl_after_completion(None, bid(3, Strain::Spades)),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♠ (2♠) 3NT -"),
        direct_three_notrump_answer(),
    ));

    entries.extend(rows_of(
        Pattern::node("P* 1NT (X)"),
        doubled_notrump_responses(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♦ (X)"),
        major_transfer_accepts(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♥ (X)"),
        major_transfer_accepts(Suit::Spades),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2♠ (X)"),
        complete_minor_transfer(Suit::Clubs),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 2NT (X)"),
        complete_minor_transfer(Suit::Diamonds),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 4♦ (X)"),
        complete_texas(Suit::Hearts),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1NT - 4♥ (X)"),
        complete_texas(Suit::Spades),
    ));

    entries.extend(rows_of(
        Pattern::node("P* 2♦ (X)"),
        two_diamond_doubled_responses(),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 2♦ (X) XX -"),
        forced(bid(2, Strain::Hearts)),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 2♦ (X) 2♥ -"),
        forced(bid(2, Strain::Spades)),
    ));

    // The new 1♦ guarantees spades. This is the one low-interference response
    // whose old PEN meaning survives unchanged: a natural three-plus-card fit.
    entries.extend(rows_of(
        Pattern::node("P* 1♦ (1♥)"),
        Rules::new().rule(
            bid(1, Strain::Spades),
            200,
            hcp(6..) & len(Suit::Spades, 3..),
        ),
    ));
    entries.extend(rows_of(
        Pattern::node("P* 1♦ (1NT)"),
        Rules::new().rule(
            bid(2, Strain::Spades),
            300,
            len(Suit::Spades, 4..) & support_points(Suit::Spades, 6..=9),
        ),
    ));

    // Only a declared natural defense activates these calls. Artificial 2M
    // conventions using the same face must not be treated as penalty targets.
    if agreements.decision.reading.notrump_defense == NotrumpDefense::Natural {
        entries.extend(rows_of(
            Pattern::node("P* 1NT (2♣)"),
            natural_minor_overcall(Suit::Clubs),
        ));
        entries.extend(rows_of(
            Pattern::node("P* 1NT (2♦)"),
            natural_minor_overcall(Suit::Diamonds),
        ));
        entries.extend(rows_of(
            Pattern::node("P* 1NT (2♥)"),
            natural_major_overcall(Suit::Hearts),
        ));
        entries.extend(rows_of(
            Pattern::node("P* 1NT (2♠)"),
            natural_major_overcall(Suit::Spades),
        ));
    }

    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-competition",
        gate: |_| true,
        entries,
    }
}
