# Preliminary BridgeTool system draft

> This is a preliminary discussion document, not a complete executable
> specification. It is intentionally not implemented in Rust yet. Meanings,
> priorities, edge cases, responses, and competitive sequences remain to be
> specified.

## Opening sketch

- **1♣:** 16+ HCP, except a 16–19 HCP hand with the special club shape assigned
  to 1♠.
- **1♦:** 10–15 HCP, 4+ spades, intended as unbalanced; may contain a longer
  club, diamond, or heart suit.
- **1♥:** 10–15 HCP, 4+ hearts, denies 4+ spades, intended as unbalanced; may
  contain a longer minor.
- **1♠:** either:
  - 10–15 HCP with at least 4–4 in the minors; or
  - 16–19 HCP with 5+ clubs, unbalanced, clubs strictly the longest suit, and
    not 5–5.
- **1NT:** 12–15 HCP, semi-balanced, with at least 3–3 in the majors. Allowed
  major-suit lengths are 3–3, 4–3, 3–4, 4–4, 5–3, and 3–5. Also includes
  4–4–4–1 hands with a singleton minor.
- **2♣:** 10–15 HCP, 5+ clubs, normally without a side suit.
- **2♦:** 10–15 HCP, 5+ diamonds, normally without a side suit.
- **2♥ / 2♠:** weak openings with 5+ cards.

## Preliminary continuation after 1♥

- **1NT:** shows 5+ spades and is forcing for one round.
- **1♠:** denies 5 spades and functions mainly as a relay.

## Open questions

- The exact definitions of *unbalanced* and *semi-balanced*.
- How to treat semi-balanced hands with at most two cards in the other major.
- The exact meaning of “normally without a side suit”.
- Opening priority when multiple descriptions fit.
- Borderline hands and possible downgrading of balanced 10–11 HCP hands.
- Position- and vulnerability-dependent variations.
- Responses, rebids, and competitive sequences not yet specified.
