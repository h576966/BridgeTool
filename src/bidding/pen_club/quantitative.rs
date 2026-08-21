//! Quantitative 4NT only at PEN's explicitly natural notrump anchors.

use crate::bidding::agreements::Agreements;
use crate::bidding::constraint::{balanced, hcp};
use crate::bidding::rows::{Entry, Package, Pattern, rows_of};
use crate::bidding::{Alert, Rules};
use contract_bridge::auction::Call;
use contract_bridge::{Bid, Strain};

const QUANTITATIVE: Alert = Alert("pen:quantitative-four-notrump");

const fn bid(level: u8, strain: Strain) -> Bid {
    Bid::new(level, strain)
}

fn ask(range: core::ops::RangeInclusive<u8>) -> Rules {
    Rules::new()
        .rule(bid(4, Strain::Notrump), 300, hcp(range) & balanced())
        .alert(QUANTITATIVE)
}

fn answer(accept: core::ops::RangeFrom<u8>, decline: core::ops::RangeInclusive<u8>) -> Rules {
    Rules::new()
        .rule(bid(6, Strain::Notrump), 200, hcp(accept))
        .rule(Call::Pass, 100, hcp(decline))
}

fn entries(_: &Agreements) -> Vec<Entry> {
    let mut entries = rows_of(Pattern::node("P* 2NT -"), ask(9..=10));
    entries.extend(rows_of(
        Pattern::node("P* 2NT - 4NT -"),
        answer(24.., 22..=23),
    ));

    entries.extend(rows_of(Pattern::node("P* 1♣ - 1NT -"), ask(19..=20)));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 1NT - 4NT -"),
        Rules::new()
            .rule(bid(6, Strain::Notrump), 200, hcp(14..))
            .rule(Call::Pass, 100, hcp(9..=11)),
    ));

    entries.extend(rows_of(Pattern::node("P* 1♣ - 1♦ - 1NT -"), ask(15..=16)));
    entries.extend(rows_of(
        Pattern::node("P* 1♣ - 1♦ - 1NT - 4NT -"),
        answer(18.., 16..=17),
    ));
    entries
}

pub(super) fn package() -> Package {
    Package {
        name: "pen-quantitative",
        gate: |_| true,
        entries,
    }
}
