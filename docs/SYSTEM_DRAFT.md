# PEN-Club — playable system draft

The partnership's intended meanings live in the human-owned
[PEN-Club system specification](pen-club-system.md). This document describes
the executable implementation, its remaining coverage gaps, and measurements.

PEN-Club is an executable Pons system, but it is not a complete convention
book. Practice, Demo, Book, and Edit → Bid it out all use the real PEN-Club
bidder. Authored nodes are followed by a PEN-only natural fallback in positions
that remain explicitly `TBD`.

The implementation lives in [`src/bidding/pen_club.rs`](../src/bidding/pen_club.rs)
and [`src/bidding/pen_club/`](../src/bidding/pen_club/). The independent
[`Opening Audit`](../src/bidding/bridge_tool.rs) retains literal eligibility,
selection, overlap, and no-match diagnostics; parity tests keep its selected
opening aligned with the executable root classifier.

## Opening system

All first-pass strength tests use raw Milton Work HCP. Preempts have the
explicit vulnerability and suit-quality adjustment described below; other
position, vulnerability, and judgment adjustments are not encoded.

The implemented priority is:

1. 1NT when its shape and 12–15 HCP range apply.
2. 1♦ with 10–15 HCP, normally an unbalanced hand, and 4+ spades; exact
   5♠–2♥–3♦–3♣ hands outside 1NT are included.
3. 1♥ with 10–15 HCP, an unbalanced hand, 4+ hearts, and fewer than four
   spades.
4. 2♣/2♦ for the exact long-minor families described below.
5. The limited minor-two-suiter 1♠.
6. The 16–19 strong-club 1♠ branch.
7. Other 16+ hands open 1♣, except balanced 22–24 hands, which open 2NT.
8. Weak 2♥/2♠ with six cards, or a three-level preempt with seven cards.

Meanings:

- **1♣:** 16+ HCP, except the strong 1♠ club branch and balanced 22–24.
- **1♦:** 10–15 HCP, normally unbalanced, 4+ spades. A longer heart or minor
  is allowed, and exact 5♠–2♥–3♦–3♣ hands outside 1NT are included.
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
- **2♥/2♠:** 5–9 HCP, 6+ cards, and A, K, or Q in the opened major.
- **2NT:** 22–24 HCP, balanced.
- **3♣/3♦/3♥/3♠:** 5–9 HCP, 7+ cards, and A, K, or Q in the opened suit.

When vulnerable against non-vulnerable opponents, both preempt families use a
7–9 HCP floor and require A, K, QJ, or QT in the long suit. Weak jump overcalls
reuse the same contextual gate. Opening Audit has no vulnerability input, so it
shows the normal/favorable 5–9 and A/K/Q eligibility; executable bidding applies
the stricter unfavorable rule at the table.

Weak 5–5+ minors do **not** open 2NT. That treatment has been considered but is
not approved.

## Authored continuations

The current authored book contains:

- the PEN 1♣ negative, positive-major, balanced, natural-minor, weak jump-major,
  and both-minor response families; the documented 1♣–1♦ rebids; positive-major
  support and side-suit rebids; and the readable parts of the positive-minor
  continuations; plus negative transfers over natural one-level interference,
  natural opener descriptions, the cooperative reopening Double, and the
  artificial 20+ reopening 2♣;
- the documented 1♦ responses, the three-card heart-support rebid, the
  passable-spade-response rebids, the 15+ balanced 3NT game action with
  three-card support, direct void splinters and RKCB, plus the natural
  spade-fit actions after `1♦ (1♥)` and `1♦ (1NT)`, with the latter using
  fit-aware support points so useful shortness can justify a competitive 2♠;
- the PEN 1♥ response ladder, cheap relay, forcing 1NT spade response, relay
  rebids, maximum rebids, raises, minor responses, and the corresponding 15+
  balanced 3NT game action with three-card heart support;
- the documented 1♠ preference, transfer, INV+ ask, GF+ shape ask,
  strong/limited answer, superaccept, immediate-interference, and 2♠
  Transfer-Rubensohl families;
- the revised 1NT Stayman answers, major/minor transfers, Texas, invitations,
  quantitative 4NT, doubled-transfer completions, and natural-overcall policy;
- 2♦ major responses, Stenberg, club force, preemptive raise, strength/support
  rebids, and the explicitly readable redouble-to-hearts / 2♥-to-spades
  transfers after `2♦ (X)`;
- mixed control cooperation after the defined splinters, RKCB 1430, queen and
  specific-king continuations, useful-void responses, Exclusion RKCB, and
  ROPI/DOPI/DEPO at their authored trump-fit anchors;
- natural direct and balancing overcalls, takeout and responsive Doubles,
  including the four-three-major shape floor for an ordinary Double of a
  natural minor and the separate raw-17+ any-shape branch,
  advances, Michaels, Unusual 2NT, defenses through four-level preempts,
  Landy-plus-natural defense to a disclosed natural 1NT, and the simple
  two-suiter-plus-natural defense to an artificial strong 1♣; plus authored
  doubled tails for natural overcalls, Landy, Michaels, and Unusual 2NT;
- system-on continuations when 1♣, 1♦, 1♥, their negative responses or relays
  are doubled, and conservative natural escapes from doubled natural
  two-level openings and overcalls.

Every artificial opening, relay, preference, transfer, and forced completion
is alerted and backed by a machine-readable constraint. Inference tests verify
that 1♣, 1♦, and 1♠ are not read as their literal suits and that transfers are
read as the target suit. Full-auction tests exercise both `Partnership` and
`Table`.

The partnership definitions are executable: `UK` is game forcing and, without
a known fit, requires a known combined minimum of at least 25 HCP; no-fit
invitations start at a known combined 22 HCP. A shared limited-opening split
classifies below 13 as minimum, 14–15 as maximum, and a 13-count as minimum
only when its two longest suits total at most nine cards. The split is used by
1♦, 1♥, limited 1♠, 2♣, and 2♦ continuations, but never by 1NT.

## PEN-safe natural fallback

An unmatched PEN opening always Passes; the fallback cannot replace PEN's
artificial opening meanings with natural 1♣, 1♦, or 1♠ calls. Other unauthored
constructive, competitive, and defensive positions use a PEN-only wrapper
around the natural candidate ladder. It consumes the authored
projections, refuses Pass below game in a known game force, and cannot originate
Double or Redouble without the corresponding authored policy. In an
uncontested game force it can originate a general slam search at a known
combined 29 HCP: show an unshown four-card side suit, otherwise rebid a shown
six-card suit, establish an eight-card fit with the minimum required support,
then use economical mixed controls and RKCB. Outside that agreement it still
cannot originate 4NT or a bid above game without an explicit quantitative or
slam trigger. The learned
American/Dutch floor is not used: its neural regime and convention card do not
describe PEN-Club's artificial 1♣, 1♦, and 1♠ openings.

In competition the fallback blocks an unauthored four-of-a-major action unless
there is a known eight-card fit with at least 25 combined raw HCP, or a known
nine-card fit while non-vulnerable against vulnerable opponents. Authored game
forces bypass that gate. Known splinter fits are retained after interference,
and “below game” is measured against their actual agreed trump so a doubled
control bid cannot be passed out below the correct game.

The web Book view displays only authored nodes from `pen_club_book`; fallback
calls are intentionally not rendered as if they were authored PEN doctrine.

## Remaining `TBD` coverage

- most of the 2♣, weak-two, and 2NT response structures;
- later 1♦, 1♥, Stayman, minor-transfer, ask, and game-placement sequences whose
  exact meanings or forcing status are not agreed;
- Transfer Rubensohl over 1NT interference and interference over later asks;
- interference above the one-level over the artificial strong 1♣;
- exact penalty-conversion gates, fit-based distributional upgrades, and
  vulnerability adjustments outside the preempt gate;
- defenses to artificial or multi-meaning opponent openings other than a
  strong 1♣;
- higher preempts above the three-level and the running-major 3NT opening.

The fallback may choose a legal natural call in these positions, but that call
is not rendered in the Book view or recorded as authored PEN doctrine.

## Historical implementation-level A/B

An initial 5,000-board seeded seat-swap pilot on commit `5f46db78`
compared the playable PEN-Club implementation with Pons American. Each pair
read the opponents from the book it actually faced. Positive IMPs would favor
PEN-Club; all eight scorer/vulnerability cells instead favored the baseline
with 95% confidence intervals clear of zero:

| Baseline | Vulnerability | Plain DD IMPs/board | Perfect-defense IMPs/board | Divergent |
| --- | --- | ---: | ---: | ---: |
| shipped American | none | −1.182 [−1.329, −1.036] | −1.650 [−1.836, −1.463] | 4,230 / 5,000 |
| shipped American | both | −1.488 [−1.681, −1.295] | −2.195 [−2.429, −1.962] | 4,208 / 5,000 |
| American + instinct floor | none | −0.843 [−0.984, −0.703] | −1.503 [−1.685, −1.321] | 4,003 / 5,000 |
| American + instinct floor | both | −1.157 [−1.343, −0.970] | −1.820 [−2.049, −1.592] | 3,987 / 5,000 |

The shipped comparison used seed `20260820`; the common-floor control used
`20260821`. The reversible-sign smoke test reproduced the same magnitude with
the opposite sign after swapping treatment and baseline. Dumps are generated
by [`examples/ab-system`](../examples/ab-system/main.rs) without DDS and scored
by the existing `bba-score` release binary under both scorers.

This measured the earlier executable implementation, not the current code or
the intrinsic merit of a completed PEN-Club card. It predates the expanded
constructive/defensive packages and the PEN-safe slam ceiling.

### Current clean-commit rerun

A fresh 5,000-board rerun per vulnerability on commit `ae736517` used seed
`1787404554`. Every table contained exactly one PEN-Club partnership and one
Pons American partnership: PEN sat N/S at table A and E/W at table B, with the
pairs' actual opponent books attached in both directions. PEN therefore never
faced another PEN partnership.

| Vulnerability | Plain DD IMPs/board | Perfect-defense IMPs/board | Divergent |
| --- | ---: | ---: | ---: |
| none | −1.016 [−1.154, −0.879] | −0.922 [−1.091, −0.753] | 4,133 / 5,000 |
| both | −1.018 [−1.197, −0.839] | −1.137 [−1.354, −0.921] | 4,112 / 5,000 |

All four cells still favor Pons American with the 95% confidence interval
clear of zero. The selected worst-tail audit is diagnostic rather than a
population sample: nine of the ten largest non-vulnerable plain-DD losses ended
with PEN declaring a doubled contract, including several artificial responses
or splinters that were passed out after interference. This points to unfinished
disturbed continuations as an important remaining implementation weakness.

The subsequent doubled-auction repair directly addresses that diagnostic tail:
system-on forcing continuations, two-suiter landing bids, conservative natural
runouts, splinter/control signoffs, and the competitive four-major gate are now
authored or enforced. A new A/B run is required before attributing any measured
strength change to those repairs.

The newer figures are numerically better than the historical pilot, especially
under perfect-defense scoring, but the runs used different commits and seeds;
that contrast is longitudinal context, not a causal A/B attribution to any one
change. It still measures the current implementation rather than the intrinsic
merit of a completed PEN-Club card.

## Open system questions

- The source does not state a tie-break for equal positive majors or equal
  transfer majors. The executable draft chooses hearts on equal length; this
  needs confirmation.
- The step mapping for the 1♣ positive-major “Marmic, stepwise singleton” line
  is not printed clearly enough to author safely.
- The disturbed-1♥ prose describes opener's later minor bids and takeout-leaning
  doubles but does not identify every auction prefix.
- After `2♦ (X)`, redouble showing hearts and 2♥ showing spades are explicit;
  “and so on through 2NT” does not unambiguously map the later calls to suits.
- The exact hand gate for a penalty double after a natural 2M overcall is not
  specified. The current provisional split uses 9+ HCP with four trumps for
  double and at most 8 HCP for the signoff-oriented other-major bid.
- Objective gates for higher preempts and a “running major” 3NT opening.
- Position, vulnerability, and suit-quality adjustments outside the confirmed
  preempt gate.
