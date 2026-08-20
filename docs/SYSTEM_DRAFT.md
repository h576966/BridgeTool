# Preliminary BridgeTool system draft

> This remains a preliminary discussion document, not a complete executable
> bidding system. The isolated opening audit below is implemented in Rust and
> exposed as a read-only web view, but it is not wired into Pons rules,
> inference, responses, rebids, or competitive auctions.

## Implemented opening audit

The [pure classifier](../src/bidding/bridge_tool.rs) records every literal
opening match before applying any priority. Eligibility and selection are
separate:

1. If 1NT is eligible, select 1NT.
2. Otherwise select the only eligible opening, if there is exactly one.
3. Report several eligible openings as an ambiguity.
4. Report an empty eligible set as no match.

Rule order has no other selection meaning. In particular, overlaps between the
minor two-suiter 1♠ and 1♦ or 1♥ remain visible and unresolved.

### Shape definitions

- A hand is **unbalanced** when its two longest suits total at least nine cards,
  or when it contains a singleton or void.
- An **ordinary balanced shape** is 4333, 4432, or 5332. These comparisons sort
  the four suit lengths and therefore do not depend on suit order.
- The additional 1NT shape is 4441 with the singleton in clubs or diamonds.
  This shape remains unbalanced under the general definition; the explicit 1NT
  priority resolves any opening overlap.

### Provisional opening meanings

- **1♣:** 16+ HCP, excluding a 16–19 HCP hand that qualifies for the
  strong-club variant of 1♠.
- **1♦:** 10–15 HCP, at least four spades, and unbalanced. A longer club,
  diamond, or heart suit is allowed.
- **1♥:** 10–15 HCP, at least four hearts, fewer than four spades, and
  unbalanced. A longer minor is allowed.
- **1♠, minor two-suiter:** 10–15 HCP with at least four clubs and four
  diamonds. It has no invented priority over a matching major-suit opening.
- **1♠, strong club hand:** 16–19 HCP, at least five clubs, unbalanced, with
  clubs strictly longer than every other suit. Equal-length 5–5 or longer
  two-suiters do not qualify for this variant.
- **1NT:** 12–15 HCP, at least three hearts and three spades, and either an
  ordinary balanced shape or 4441 with a singleton minor. The permitted
  heart–spade length pairs are 3–3, 4–3, 3–4, 4–4, 5–3, and 3–5.
- **2♣ / 2♦, basic case:** 10–15 HCP, at least five cards in the opening minor,
  and every side suit shorter than four cards.

The possible 2♣/2♦ exception is a separate diagnostic, not opening eligibility:
six or more cards in the proposed minor, exactly four cards in the other minor,
and no four-card major. The diagnostic is deliberately shape-only because no
HCP rule was stated for the exception itself. “Good six-card suit” and “weak
four-card side suit” remain undefined, so the classifier does not decide them.

The audit uses raw Milton Work HCP only. It also exposes total HCP, suit lengths,
HCP by suit, sorted shape, singleton suits, and void suits so honour
concentration remains visible. Later versions may upgrade or downgrade hands
for distribution, length, suit quality, or honour location; no such formula is
implemented now.

**2♥ and 2♠ are not implemented:** their exact strength range is still
undefined.

The [probe program](../examples/probe-bridge-openings/main.rs) records its
version, seed, and hand count; reports eligibility, selection, gaps, overlaps,
and exception candidates; and retains only three sample hands per category.
The web app's Opening Audit view calls the same pure classifier through a thin
WASM JSON wrapper. It does not change the American system used by Practice or
Demo.

## Preliminary continuation after 1♥

- **1NT:** shows 5+ spades and is forcing for one round.
- **1♠:** denies 5 spades and functions mainly as a relay.

## Open questions

- Whether these audit-only shape definitions should become final system
  meanings.
- Opening priority for overlaps not resolved by the explicit 1NT priority.
- Whether the 6–4 minor exception has an HCP band.
- Objective definitions of a “good six-card suit” and a “weak four-card side
  suit”.
- Exact strength ranges and shapes for 2♥ and 2♠.
- Borderline hands and possible downgrading of balanced 10–11 HCP hands.
- Position- and vulnerability-dependent variations.
- Responses, rebids, and competitive sequences not yet specified.
