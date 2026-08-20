# PEN-Club — playable system draft

PEN-Club is an executable Pons system, but it is not a complete convention
book. Practice, Demo, Book, and Edit → Bid it out all use the real PEN-Club
bidder. Authored nodes are followed by the deterministic Pons instinct floor in
positions that have not yet been specified.

The implementation lives in [`src/bidding/pen_club.rs`](../src/bidding/pen_club.rs)
and [`src/bidding/pen_club/`](../src/bidding/pen_club/). The independent
[`Opening Audit`](../src/bidding/bridge_tool.rs) retains literal eligibility,
selection, overlap, and no-match diagnostics; parity tests keep its selected
opening aligned with the executable root classifier.

## Opening system

All first-pass strength tests use raw Milton Work HCP. No position,
vulnerability, suit-quality, or judgment adjustment is encoded yet.

The implemented priority is:

1. 1NT when its shape and 12–15 HCP range apply.
2. 1♦ with 10–15 HCP, an unbalanced hand, and 4+ spades.
3. 1♥ with 10–15 HCP, an unbalanced hand, 4+ hearts, and fewer than four
   spades.
4. 2♣/2♦ for the exact long-minor families described below.
5. The limited minor-two-suiter 1♠.
6. The 16–19 strong-club 1♠ branch.
7. Other 16+ hands open 1♣, except balanced 22–24 hands, which open 2NT.

Meanings:

- **1♣:** 16+ HCP, except the strong 1♠ club branch and balanced 22–24.
- **1♦:** 10–15 HCP, unbalanced, 4+ spades. A longer heart or minor is allowed.
- **1♥:** 10–15 HCP, unbalanced, 4+ hearts and fewer than four spades. A
  longer minor is allowed.
- **1♠, limited branch:** 10–15 HCP, 4+ clubs and 4+ diamonds, and fewer than
  four cards in each major. This explicit major denial is important: for
  example `AKQ2.3.KQJ2.8765` opens 1♦, never 1♠. Exact 6+ clubs/4 diamonds and
  4 clubs/6+ diamonds hands go to 2♣/2♦; 5–5+ minors remain in 1♠.
- **1♠, strong branch:** 16–19 HCP, unbalanced, 5+ clubs, with clubs strictly
  longer than every other suit. Equal-length strong two-suiters remain in 1♣.
  Both branches are one alerted union rule.
- **1NT:** 12–15 HCP, at least three hearts and three spades, and either 4333,
  4432, 5332, or 4441 with a singleton minor.
- **2♣:** 11–15 HCP, 6+ clubs, no four-card major, and at most four diamonds.
- **2♦:** 11–15 HCP, 6+ diamonds, no four-card major, and at most four clubs.
- **2♥/2♠:** 5–9 HCP and 6+ cards in the opened major.
- **2NT:** 22–24 HCP, balanced.

Weak 5–5+ minors do **not** open 2NT. That treatment has been considered but is
not approved.

## Authored continuations

The current authored book contains:

- the PEN 1♣ negative, positive-major, balanced, natural-minor, weak jump-major,
  and both-minor response families; the documented 1♣–1♦ rebids; positive-major
  support and side-suit rebids; and the readable parts of the positive-minor
  continuations;
- the portions of the old 1♦ response shell that remain meaningful after 1♦
  became an unbalanced spade-showing opening, plus a natural spade-fit action
  after `1♦ (1♥)`; no old “balanced versus spade hand” split remains;
- the PEN 1♥ response ladder, cheap relay, forcing 1NT spade response, relay
  rebids, maximum rebids, raises, and minor responses;
- the known 1♠ response core: passable 1NT/2♣ minor preferences and 2♦/2♥
  transfers to hearts/spades, including system-on responses after a double or
  cheap 2♣ overcall and completion when a transfer is doubled;
- 1NT Stayman, major transfers, pure club/diamond transfers, Stayman answers,
  transfer accepts, minor completions, and the provisional natural-2M overcall
  policy;
- 2♦ major responses, Stenberg, club force, preemptive raise, strength/support
  rebids, and the explicitly readable redouble-to-hearts / 2♥-to-spades
  transfers after `2♦ (X)`.

Every artificial opening, relay, preference, transfer, and forced completion
is alerted and backed by a projected constraint. Inference tests verify that
1♣, 1♦, and 1♠ are not read as their literal suits and that transfers are read
as the target suit. Full-auction tests exercise both `Partnership` and `Table`.

The two PEN response abbreviations need numerical gates in an executable
classifier, but the source gives labels rather than HCP tables. This draft uses
9+ after strong 1♣ (the complement of the explicit 0–8 negative), 12+ for `UK`
after limited openings, and 10+ for `INV+`. These are visible provisional
implementation choices, not claims about an approved final card.

## Deterministic floor

Unauthored constructive, competitive, and defensive positions fall through to
the same deterministic `instinct` ladder. The learned American/Dutch floor is
not used: its neural regime and convention card do not describe PEN-Club's
artificial 1♣, 1♦, and 1♠ openings. The deterministic floor consumes the
authored projections, so it sees the shown spades, minors, and transfer target
rather than literal call names.

The web Book view displays only authored nodes from `pen_club_book`; the floor
is intentionally not rendered as if it were authored PEN doctrine.

## Positions still owned by the floor

- most continuations after the redefined 1♦ opening;
- most competition outside the specifically authored 1♦, 1♠, 1NT, and 2♦
  tails;
- defensive bidding after the opponents open;
- 1♠ responses 2♠ and 2NT;
- later Stayman, transfer, slam, and game-placement sequences not explicitly
  authored here;
- higher preempts and the running-major 3NT opening, because the source names
  them but does not give an objective strength/suit-quality gate.

## Open system questions

- The source does not state a tie-break for equal positive majors or equal
  transfer majors. The executable draft chooses hearts on equal length; this
  needs confirmation.
- Exact numeric definitions of `UK`, `INV`, `INV+`, “maximum”, and “minimum”
  outside ranges printed in the source need confirmation.
- The step mapping for the 1♣ positive-major “Marmic, stepwise singleton” line
  is not printed clearly enough to author safely.
- The disturbed-1♥ prose describes opener's later minor bids and takeout-leaning
  doubles but does not identify every auction prefix.
- After `2♦ (X)`, redouble showing hearts and 2♥ showing spades are explicit;
  “and so on through 2NT” does not unambiguously map the later calls to suits.
- The exact hand gate for a penalty double after a natural 2M overcall is not
  specified. The current provisional split uses 9+ HCP with four trumps for
  double and at most 8 HCP for the signoff-oriented other-major bid.
- Objective gates for three-level preempts and a “running major” 3NT opening.
- Position, vulnerability, and suit-quality adjustments.
