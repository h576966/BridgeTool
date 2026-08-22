# PEN-Club system specification

This is the human-owned, living specification for PEN-Club. It describes what
the partnership intends to play. The separate
[implementation draft](SYSTEM_DRAFT.md) records what BridgeTool currently
implements and how that implementation measures.

The system owner is authoritative. BridgeTool must not silently resolve entries
marked **TBD**.

## Status legend

- **Confirmed** — explicitly confirmed by the system owner.
- **Derived** — follows mechanically from a confirmed agreement.
- **Draft** — represented in the current executable system, but still needs
  confirmation as partnership doctrine.
- **TBD** — not yet specified.

## Terminology and general agreements

### Hand valuation

- Opening ranges currently use Milton Work high-card points (**HCP**).
- Once an eight-card or longer trump fit is known, evaluate the hand using HCP,
  trump length, and distribution. The exact distributional scale is **TBD**.
- Position and judgment adjustments remain **TBD** except for the explicit
  preempt vulnerability and suit-quality agreement below.

### Forcing terms

- **UK / GF (game forcing) — Confirmed.** The partnership may not stop below
  game. Use this sparingly while no common eight-card trump fit is known. In
  that case the auction must establish a known combined minimum of at least
  25 HCP.
- **INV (invitational) — Confirmed principle.** Use when either:
  - a good trump fit is established and game has a good chance; or
  - the partnership is known to hold roughly 22–28 combined HCP and game is
    good only when partner is near the top of the range already shown.
  INV is therefore contextual rather than one universal HCP interval.
- **INV+ — Confirmed.** At least invitational, but may contain a stronger hand.
- **Minimum — Confirmed.** Shows the lower end of the range established by
  earlier calls. It is a negative action and is almost always passable unless
  it is explicitly a relay.
- **Maximum — Confirmed.** Shows the upper end of the range established by
  earlier calls. It is a positive action, but is not automatically game
  forcing.

### Unspecified auctions

- **Confirmed policy:** use natural bidding when a continuation has not yet
  been specified.
- The goal is still to author as much of the system as possible. “Natural” is
  not permission to invent an undisclosed convention or an aggressive generic
  slam action.
- Exact natural fallback ranges and forcing rules are **TBD**.

## Opening structure

The current opening priority is 1NT, 1♦, 1♥, long-minor 2♣/2♦, limited 1♠,
strong-club-branch 1♠, 1♣, preempts, and 2NT. A seven-card preemptive suit uses
the three-level rather than a weak two. Priority matters where shapes overlap.

| Call | Meaning | Status |
| --- | --- | --- |
| Pass | Opening requirements not met | Natural details TBD |
| 1♣ | 16+ HCP, except the strong 1♠ club branch and balanced 22–24 | Draft |
| 1♦ | 10–15 HCP, normally unbalanced, 4+ spades; a longer heart or minor is allowed, and 5♠–2♥–3♦–3♣ hands outside the 1NT opening are included | Draft |
| 1♥ | 10–15 HCP, unbalanced, 4+ hearts and fewer than four spades; a longer minor is allowed | Draft |
| 1♠ | Either the limited two-minor branch or the strong club branch described below | Draft, artificial |
| 1NT | 12–15 HCP, 3+ cards in both majors; 4333, 4432, 5332, or 4441 with a singleton minor | Confirmed |
| 2♣ | 11–15 HCP, 6+ clubs, no four-card major, at most four diamonds | Draft |
| 2♦ | 11–15 HCP, 6+ diamonds, no four-card major, at most four clubs | Draft |
| 2♥ / 2♠ | 5–9 HCP, 6+ cards, and A, K, or Q in the opened major; unfavorable vulnerability uses the stricter rule below | Confirmed |
| 2NT | 22–24 HCP, balanced | Draft |
| 3♣ / 3♦ / 3♥ / 3♠ | 5–9 HCP, 7+ cards, and the same suit-quality/vulnerability rule as a weak two | Confirmed |
| 3NT | A running major | Exact requirements TBD |
| Higher openings | — | TBD |

At normal or favorable vulnerability, a weak two or three-level preempt needs
at least one of the ace, king, or queen in its suit. Vulnerable against
non-vulnerable opponents, it needs 7–9 HCP and either the ace, the king, or
queen-jack/queen-ten in the suit. Thus `Kxxxxx` is acceptable at the stricter
vulnerability when the hand has 7–9 HCP; bare `Qxxxxx` is not. The same gate
applies to weak jump overcalls.

### The two meanings of 1♠

- **Limited branch — Draft:** 10–15 HCP, 4+ clubs and 4+ diamonds, fewer
  than four hearts and fewer than four spades.
  - Exact 6+ clubs with four diamonds opens 2♣.
  - Exact 6+ diamonds with four clubs opens 2♦.
  - 5–5 or longer minors remain in 1♠.
  - A four-card major excludes this branch. For example,
    `AKQ2.3.KQJ2.8765` opens 1♦, not 1♠.
- **Strong branch — Draft:** 16–19 HCP, unbalanced, 5+ clubs, with clubs
  strictly longer than every other suit. Equal-length strong two-suiters open
  1♣.

## Responses and continuations

### After 1♣

**Confirmed forcing rule:** every response is game forcing except 1♦ and the
weak jump responses 2♥/2♠. A positive response should be chosen carefully: game
must be decent opposite a balanced 16 HCP minimum.

#### Responses

| Auction | Meaning | Force | Status |
| --- | --- | --- | --- |
| 1♣–1♦ | Negative, currently 0–8 HCP | Not GF | Confirmed structure; range Draft |
| 1♣–1♥ | Positive hearts: 5+ hearts, or exactly four with a Marmic hand or a five-card minor; currently 9+ HCP | GF | Draft |
| 1♣–1♠ | Positive spades: 5+ spades, or exactly four with a Marmic hand or a five-card minor; currently 9+ HCP | GF | Draft |
| 1♣–1NT | Balanced; currently 9–11 or 14+ HCP | GF | Draft |
| 1♣–2♣ | Natural, 6+ clubs; currently 9+ HCP | GF | Draft |
| 1♣–2♦ | Natural, 6+ diamonds; currently 9+ HCP | GF | Draft |
| 1♣–2♥ / 2♠ | Weak jump, 6+ cards, currently 5–8 HCP | Not GF | Draft |
| 1♣–2NT | Balanced, currently 12–13 HCP | GF | Draft |
| 1♣–3♣ | Both minors, 5–5 or longer; currently 9+ HCP | GF | Draft |
| Other responses | — | TBD |

With equal positive majors, the current implementation chooses hearts. The
partnership tie-break is **TBD**.

#### After a natural one-level overcall

Responder uses negative transfers with 0–8 HCP and 5+ cards in the shown suit.
The transfer forces opener to act once, but opener need not complete it and may
show a better natural suit instead.

| Overcall | Double | Next call | Following call |
| --- | --- | --- | --- |
| 1♦ | 5+ hearts | 1♥ = 5+ spades | 1♠ = 5+ clubs |
| 1♥ | 5+ spades | 1♠ = 5+ clubs | 1NT = 5+ diamonds |
| 1♠ | 5+ hearts | 1NT = 5+ clubs | 2♣ = 5+ diamonds |

When several negative suits qualify, the current draft shows the longest and
uses the cheapest transfer on an equal-length tie. Pass shows 0–8 without a
five-card unbid suit, or a positive hand suitable for converting a reopening
Double for penalties. A passed negative is forcing on opener when fourth hand
Passes: the opponents' one-level overcall may not be passed out.

Natural positive suit calls remain 9+ HCP and game forcing where space permits.
The cue-bid is the 9+ game-forcing catch-all when interference displaces the
normal positive response; 2NT is natural and positive with a stopper where the
transfer ladder consumes 1NT.

After a negative transfer, opener normally completes with support, bids a
five-card or longer side suit naturally, or bids notrump with a balanced hand
and a stopper. A cheap completion is forced when no better description exists
and therefore does not promise three-card support. Except when 2♣ completes a
club transfer, an available 2♣ is an artificial 20+ forcing relay.

After `1♣–(one-level suit)–Pass–Pass`, opener uses:

| Call | Meaning | Force |
| --- | --- | --- |
| Double | Cooperative/takeout, normally 16–19; responder may convert with a penalty hand | Not forcing |
| 1NT | 16–19 balanced with a stopper | Not forcing |
| New suit other than clubs | Natural, 16–19, 5+ cards; commonly 6+ at the two-level | Not forcing |
| 2♣ | Artificial 20+ hand, any shape | Forcing one round |

The natural 2♣ rebid is not needed here: a 16–19 hand with clubs strictly
longest opens PEN's strong 1♠ branch, while an equal-length club two-suiter can
show its other suit. A 20+ club hand starts with the artificial 2♣ relay and
shows clubs later.

#### Opener after the negative 1♦

| Auction | Current meaning | Status |
| --- | --- | --- |
| 1♣–1♦; 1♥ / 1♠ | 16–21 HCP, 4+ cards in the major | Draft |
| 1♣–1♦; 1NT | 16–18 HCP, balanced, no five-card major | Draft |
| 1♣–1♦; 2♣ | 19–21 HCP, balanced, artificial force | Draft |
| 1♣–1♦; 2♦ | 16–21 HCP, 6+ diamonds | Draft |
| 1♣–1♦; 2♥ / 2♠ | 19–21 HCP, 6+ cards in the major | Draft |
| 1♣–1♦; 2NT | 25+ HCP, balanced | Draft |
| 1♣–1♦; 3♣ | 16–19 HCP, 5+ clubs and 5+ diamonds | Draft |
| Other rebids and continuations | — | TBD |

#### Opener after a positive major

- Raise to 2M with 3+ support.
- Show 5+ clubs or diamonds naturally; 3♣ currently shows both minors.
- After 1♥, 1♠ currently shows 4+ spades. After 1♠, 2♥ currently shows
  5+ hearts.
- 1NT currently shows a balanced or Marmic hand with fewer than three cards in
  responder’s major.
- After the raise to 2M, responder’s 2NT is currently Stenberg with 5+ cards in
  the major; 3♣/3♦ show a five-card side suit with 4+ trumps.
- Later game and slam placement is **TBD**.

#### Opener after a positive natural minor

- 3m: 3+ support.
- 3NT: currently balanced 16–18 with 3+ support.
- 2NT: balanced or Marmic with fewer than three cards in responder’s minor.
- Other suit: natural, currently 5+ cards.
- Later continuations are **TBD**.

### After 1♦ — 10–15, normally unbalanced, 4+ spades

The opening also includes 5♠–2♥–3♦–3♣ hands, which cannot open 1NT because
that opening requires at least three cards in each major. Whether any other
balanced-looking exceptions belong in 1♦ is **TBD**.

#### Existing response structure

| Auction | Current meaning | Force/status |
| --- | --- | --- |
| 1♦–1♥ | Natural, 4+ hearts | Draft |
| 1♦–1♠ | Spade support, 3+ spades; passable and usually exactly three spades, since four-card support normally responds 2♠ or higher | Draft; exact four-card exceptions TBD |
| 1♦–1NT | Limited, fewer than four hearts and fewer than three spades; potentially up to about 13 HCP | Draft; passable, though opener will rarely pass |
| 1♦–2♣ / 2♦ | Natural, 4+ cards | **GF Confirmed**; absent a known fit, the 10-HCP opening minimum derives a responder minimum of 15 HCP |
| 1♦–2♥ | Weak, 6(7)+ hearts and fewer than three spades | Draft; exact range TBD |
| 1♦–2♠ | 4+ spade support, about 6–9 HCP | Draft |
| 1♦–2NT | Proposed Jacoby/Stenberg raise: INV+ with 4+ spade support | **TBD**; proposal not yet confirmed |
| 1♦–3♣ / 3♦ / 3♥ | Either a natural invitation with a good 6+ suit or a mini-splinter | **TBD**; choice not agreed |

The exact boundary between very weak four-card spade raises using 1♠ and 2♠
is **TBD**.

#### Provisional opener rebid after 1♥

After `1♦–1♥`, opener's 1NT rebid provisionally promises exactly three-card
heart support. Other opener rebids and later continuations are **TBD**.

#### Opener after the passable 1♠ response

Opener may pass 1♠. The currently defined continuations are:

| Auction | Current meaning | Force/status |
| --- | --- | --- |
| 1♦–1♠; 1NT | Maximum 14–15 HCP with exactly five spades and no better descriptive rebid; this can include 5♠–2♥–3♦–3♣ | Draft; provisional, later continuations TBD |
| 1♦–1♠; 2♣ / 2♦ / 2♥ | Exactly four spades and 5+ cards in the bid side suit | Draft; forcing status TBD |

The treatment of other hands with five spades and a four-card or longer side
suit is not complete. The provisional 1NT rebid covers maximum hands only when
there is no better descriptive rebid; the allocation of other such hands
between Pass and 2♠, and the exact meaning of 2♠, are **TBD**.

#### Confirmed direct game/slam actions

- **3NT:** three-card spade support and a fairly balanced hand. Exact strength
  and whether controls/stopper quality matter are **TBD**.
- **4♣ / 4♦:** void in the bid suit, spade support, and either a serious slam
  try or a hand comfortable passing if opener signs off in spades.
- **4♥:** the same splinter principle: heart void, spade support, serious slam
  interest or willingness to pass a spade signoff.
- **4NT:** RKCB 1430 for spades.
- The meaning of a direct **4♠** is **TBD**.

After `1♦ (1♥)`, 1♠ is currently natural with 3+ spade support and 6+ HCP.
After `1♦ (1NT)`, 2♠ is a natural, nonforcing competitive raise with 4+
spades and 6–9 fit-aware support points. Once opener's 4+ spades establish the
fit, side-suit shortness counts: for example, 4 HCP plus a useful void may be
enough for 2♠. Whether 3♠ should show five-card support or also allow
four-card support with extreme shape is **TBD**. Other contested sequences and
most opener rebids are **TBD**.

### After 1♥ — 10–15, unbalanced, 4+ hearts, fewer than four spades

#### Existing response structure

| Auction | Current meaning | Force/status |
| --- | --- | --- |
| 1♥–1♠ | Cheap artificial relay; currently at most 10 HCP and fewer than five spades | Draft |
| 1♥–1NT | Forcing response showing 5+ spades | Draft |
| 1♥–2♣ / 2♦ | Natural, 4+ cards | **GF Confirmed**; absent a known fit, the 10-HCP opening minimum derives a responder minimum of 15 HCP |
| 1♥–2♥ | Exactly three-card support, currently 10–12 HCP | Draft |
| 1♥–2♠ | Weak jump, 6+ spades, currently 5–8 HCP | Draft |
| 1♥–2NT | Stenberg, 4+ heart support, currently 11+ HCP | Draft |
| 1♥–3♣ / 3♦ | 6+ cards, currently 10–11 HCP | Draft; confirm meaning |
| 1♥–3♥ | 4+ heart support, currently 7–10 HCP | Draft |

#### Confirmed direct game/slam actions

- **3NT:** three-card heart support and a fairly balanced hand. Exact strength
  is **TBD**.
- **4♣ / 4♦:** void in the bid suit, heart support, and either a serious slam
  try or a hand comfortable passing if opener signs off in hearts.
- **4NT:** RKCB 1430 for hearts.
- The spade-void splinter, if any, and the meanings of direct 3♠ and 4♠ are
  **TBD**.

#### Current opener rebids after the 1♠ relay

- 1NT is the low/default heart rebid.
- 2♣/2♦ show a five-card minor with 4+ hearts.
- Maximum actions currently use 14–15 HCP:
  - 2♥: 6+ hearts;
  - 2♠: 5+ hearts and 5+ clubs;
  - 2NT: 5+ hearts and 5+ diamonds;
  - 3♣/3♦: 6+ in the minor with 4+ hearts;
  - 3♥: 7+ hearts;
  - 3♠: 0–4–4–4 “super-Marmic”.
- All meanings and follow-ups still need confirmation.

#### Current opener rebids after the forcing 1NT spade response

- 2♥: 5+ hearts, default/minimum action.
- 2♣/2♦: 5+ minor with 4+ hearts.
- 2♠: 11–13 HCP with 3+ spades.
- Maximum 14–15 actions currently include 2NT without three-card spade support,
  3♣/3♦ with a six-card minor, and 3♠ with 3+ spades.
- The exact 3♥ meaning and all later placement are **TBD**.

### After 1♠ — artificial two-branch opening

The opening is forcing for one round; responder may not pass. The limited
branch is 10–15 HCP with both minors, while the strong branch is the 16–19 HCP
club hand defined above.

| Response | Current meaning | Status |
| --- | --- | --- |
| 1NT | Passable preference for diamonds; diamonds at least as long as clubs | Draft |
| 2♣ | Passable preference for clubs; clubs longer than diamonds | Draft |
| 2♦ | Transfer to hearts, 5+ hearts; may be weak or unlimited | Draft |
| 2♥ | Transfer to spades, 5+ spades; may be weak or unlimited | Draft |
| 2♠ | INV+ general hand-family ask | Draft |
| 2NT | GF+ detailed shape ask | Draft |
| Higher responses | — | TBD |

For strong hands with more than one possible descriptive rebid, the priority is
support for responder's transferred major, then hearts, spades, diamonds, and
finally the one-suited club hand. Thus a strong response showing spades denies
four hearts, and one showing diamonds denies a four-card major. Later relays
may uncover an additional side suit.

The current responder tie-break transfers to hearts with equal 5–5 majors.
Confirm or replace this rule.

#### Opener after the passable 1NT preference

| Auction | Current meaning | Status |
| --- | --- | --- |
| 1♠–1NT; Pass | Limited branch, content to play 1NT | Draft |
| 1♠–1NT; 2♣ | Limited branch; clubs are the longer or better minor | Draft |
| 1♠–1NT; 2♦ | Limited branch; diamonds are at least as long as clubs, to play | Draft |
| 1♠–1NT; 2♥ | Strong branch with 4+ hearts | Draft |
| 1♠–1NT; 2♠ | Strong branch with 4+ spades, denying four hearts | Draft |
| 1♠–1NT; 2NT | Strong branch with 4+ diamonds, denying a four-card major | Draft |
| 1♠–1NT; 3♣ | Strong branch without a four-card side suit; consequently 6+ clubs | Draft |
| 1♠–1NT; 3♦ | Limited branch, maximum by playing strength, normally 14–15 HCP, with 5+ diamonds | Draft |

The strong branch must rebid and may not use Pass, 2♣, 2♦, or 3♦. Its natural
shape rebids do not independently create a game force and may be passed by a
very weak responder with a suitable fit.

#### Opener after the passable 2♣ preference

| Auction | Current meaning | Status |
| --- | --- | --- |
| 1♠–2♣; Pass | Limited branch, content to play 2♣ | Draft |
| 1♠–2♣; 2♦ | Strong branch with 4+ diamonds, denying a four-card major | Draft |
| 1♠–2♣; 2♥ | Strong branch with 4+ hearts | Draft |
| 1♠–2♣; 2♠ | Strong branch with 4+ spades, denying four hearts | Draft |
| 1♠–2♣; 2NT | Strong branch without a four-card side suit; consequently 6+ clubs | Draft; forces 3♣ from a weak responder |
| 1♠–2♣; 3♣ | Limited branch, maximum by playing strength, normally 14–15 HCP, with 5+ clubs | Draft |

The strong branch must rebid. After its artificial 2NT rebid, responder's 3♣
is the weak signoff; other continuations show constructive or better values
and are **TBD**.

#### The INV+ 2♠ general ask

| Auction | Current meaning | Force/status |
| --- | --- | --- |
| 1♠–2♠; 2NT | Limited branch, diamonds at least as long as clubs | Passable with only invitational values |
| 1♠–2♠; 3♣ | Limited branch, clubs longer than diamonds | Passable with only invitational values |
| 1♠–2♠; 3♦ | Strong branch with 4+ diamonds, denying a four-card major | GF |
| 1♠–2♠; 3♥ | Strong branch with 4+ hearts | GF |
| 1♠–2♠; 3♠ | Strong branch with 4+ spades, denying four hearts | GF |
| 1♠–2♠; 3NT | Strong branch without a four-card side suit; consequently 6+ clubs | GF |

The 2♠ ask is forcing through opener's first answer. Detailed continuations
after every answer are **TBD**.

#### The GF+ 2NT detailed shape ask

| Auction | Current meaning | Force/status |
| --- | --- | --- |
| 1♠–2NT; 3♣ | Limited branch, clubs longer than diamonds | GF |
| 1♠–2NT; 3♦ | Limited branch, diamonds at least as long as clubs | GF |
| 1♠–2NT; 3♥ | Strong branch with 4+ hearts | GF |
| 1♠–2NT; 3♠ | Strong branch with 4+ spades, denying four hearts | GF |
| 1♠–2NT; 3NT | Strong branch without a four-card major: either 4+ diamonds or a one-suited club hand | GF; may be passed with game-only values |

After `1♠–2NT; 3NT`, 4♣ is a slam-interest relay: 4♦ shows the strong
club-and-diamond hand, while 4♥ shows the one-suited club hand without a
four-card side suit. Elsewhere the cheapest relay asks for further shape or
shortness; the exact step mappings are **TBD**.

#### Opener after the heart transfer

The two-level completion is mandatory for the limited branch only. The strong
branch always breaks the transfer.

| Auction | Current meaning | Status |
| --- | --- | --- |
| 1♠–2♦; 2♥ | Limited branch, mandatory completion | Draft; passable |
| 1♠–2♦; 2♠ | Strong branch with 4+ spades and fewer than three hearts | Draft |
| 1♠–2♦; 2NT | Strong branch with 3+ heart support | Draft; forces 3♥ |
| 1♠–2♦; 3♣ | Strong branch with 6+ clubs, no four-card side suit, and fewer than three hearts | Draft |
| 1♠–2♦; 3♦ | Strong branch with 4+ diamonds, fewer than three hearts, and fewer than four spades | Draft |
| 1♠–2♦; 3♥ | Limited branch superaccept: exactly three hearts and a maximum by playing strength, normally 14–15 HCP | Draft |

After the strong 2NT support rebid, responder's 3♥ is the weak signoff. Other
continuations show constructive or better values and are **TBD**. Natural
strong-branch transfer breaks may be passed by a weak responder with a suitable
fit.

#### Opener after the spade transfer

The two-level completion is mandatory for the limited branch only. The strong
branch always breaks the transfer.

| Auction | Current meaning | Status |
| --- | --- | --- |
| 1♠–2♥; 2♠ | Limited branch, mandatory completion | Draft; passable |
| 1♠–2♥; 2NT | Strong branch with 3+ spade support | Draft; forces 3♠ |
| 1♠–2♥; 3♣ | Strong branch with 6+ clubs, no four-card side suit, and fewer than three spades | Draft |
| 1♠–2♥; 3♦ | Strong branch with 4+ diamonds, fewer than three spades, and fewer than four hearts | Draft |
| 1♠–2♥; 3♥ | Strong branch with 4+ hearts and fewer than three spades | Draft |
| 1♠–2♥; 3♠ | Limited branch superaccept: exactly three spades and a maximum by playing strength, normally 14–15 HCP | Draft |

After the strong 2NT support rebid, responder's 3♠ is the weak signoff. Other
continuations show constructive or better values and are **TBD**. Natural
strong-branch transfer breaks may be passed by a weak responder with a suitable
fit.

#### Immediate interference over 1♠

Explicit competitive meanings below override the general double principles
later in this document.

After a double, the normal response structure remains on. Pass is rare and
business-oriented, showing willingness to play 1♠ doubled. Redouble replaces
the INV+ general ask, while a direct 2♠ retains the same meaning despite the
redundancy.

| Auction | Current meaning | Force/status |
| --- | --- | --- |
| 1♠–(X)–XX; 1NT | Limited branch, diamonds at least as long as clubs | Passable with only invitational values |
| 1♠–(X)–XX; 2♣ | Limited branch, clubs longer than diamonds | Passable with only invitational values |
| 1♠–(X)–XX; 2♦ | Strong branch with 4+ diamonds, denying a four-card major | GF |
| 1♠–(X)–XX; 2♥ | Strong branch with 4+ hearts | GF |
| 1♠–(X)–XX; 2♠ | Strong branch with 4+ spades, denying four hearts | GF |
| 1♠–(X)–XX; 2NT | Strong branch without a four-card side suit; consequently 6+ clubs | GF |

After a natural 1NT overcall:

| Responder | Meaning |
| --- | --- |
| Pass | Weak or no action; commonly a hand that would have responded 1NT without interference |
| Double | Values, penalty-oriented |
| 2♣ | Normal passable club preference |
| 2♦ / 2♥ | Normal major transfers |
| 2♠ | INV+ general ask |
| 2NT | GF+ detailed shape ask |

After a natural 2♣ overcall:

| Responder | Meaning |
| --- | --- |
| Pass | Weak or no action, or content to defend clubs |
| Double | Cooperative penalty; opener is already known to hold 4+ clubs |
| 2♦ / 2♥ | Major transfers |
| 2♠ | INV+ general ask |
| 2NT | GF+ detailed shape ask |

When a natural 2♦ or 2♥ overcall takes one of the transfers, Double replaces
the stolen call exactly. Other available system calls remain on:

| Auction | Pass | Double | Other calls |
| --- | --- | --- | --- |
| 1♠–(2♦) | Weak or no action | Transfer to hearts, 5+ hearts, weak or unlimited | 2♥ transfers to spades; 2♠ and 2NT retain their asks |
| 1♠–(2♥) | Weak or no action | Transfer to spades, 5+ spades, weak or unlimited | 2♠ and 2NT retain their asks |

Opener treats a transfer Double exactly like the corresponding uncontested
transfer: the limited branch completes or superaccepts, while the strong
branch uses the normal descriptive break. These transfer Doubles are never
penalty.

#### Transfer Rubensohl after a natural 2♠ overcall

| Responder | Meaning |
| --- | --- |
| Pass | Weak or no action |
| Double | INV+ general PEN ask |
| 2NT | Transfer to clubs; weak or GF |
| 3♣ | Transfer to diamonds; weak or GF |
| 3♦ | Transfer to hearts; weak or GF |
| 3♥ | Natural invitation with 6+ hearts |
| 3♠ | Transfer to 3NT, showing a spade stopper |
| 3NT | To play, denying a spade stopper |

The general-ask Double receives the normal answers:

| Auction | Current meaning | Force/status |
| --- | --- | --- |
| 1♠–(2♠)–X; 2NT | Limited branch, diamonds at least as long as clubs | Passable with only invitational values |
| 1♠–(2♠)–X; 3♣ | Limited branch, clubs longer than diamonds | Passable with only invitational values |
| 1♠–(2♠)–X; 3♦ | Strong branch with 4+ diamonds, denying a four-card major | GF |
| 1♠–(2♠)–X; 3♥ | Strong branch with 4+ hearts | GF |
| 1♠–(2♠)–X; 3♠ | Strong branch with 4+ spades, denying four hearts | GF; artificial cue-bid |
| 1♠–(2♠)–X; 3NT | Strong branch without a four-card side suit; consequently 6+ clubs | GF |

After the limited 2NT answer, 3♦ is a diamond signoff; after the limited 3♣
answer, Pass signs off in clubs. Responder may instead pass 2NT when that is
the preferred contract. The cheapest unused continuation—3♣ over 2NT or 3♦
over 3♣—is the GF relay.

Opener completes the Rubensohl transfers to 3♣, 3♦, or 3♥ regardless of
branch, including when the transfer is doubled. This is a competition-specific
exception that protects a weak responder. Pass after completion is weak;
raising a completed minor transfer to four of the minor is invitational. A GF
responder continues with the cheapest new bid to ask opener first to distinguish
the limited and strong branches. Exact later relays are **TBD**.

After the 3♠ transfer to 3NT, opener completes because responder has shown a
spade stopper. After a direct 3NT, opener passes with a spade stopper and
removes naturally without one.

Interference over these competitive transfers and asks, and action over a
natural 2NT or a three-level overcall, remain **TBD**.

### After 1NT — 12–15

| Response | Current meaning | Status |
| --- | --- | --- |
| Pass | Natural signoff | Confirmed |
| 2♣ | Stayman or a balanced invitation; may also be weak with both majors | Confirmed |
| 2♦ | Transfer to hearts, 5+ hearts, weak or unlimited | Confirmed |
| 2♥ | Transfer to spades, 5+ spades, weak or unlimited | Confirmed |
| 2♠ | Transfer to clubs, normally 6+ clubs, weak or GF | Confirmed |
| 2NT | Transfer to diamonds, normally 6+ diamonds, weak or GF | Confirmed |
| 3♣ / 3♦ | Natural invitation with a good 6+ suit | Confirmed |
| 3♥ / 3♠ | — | TBD |
| 3NT | To play, approximately 13–17 HCP | Confirmed; judgment may adjust the range |
| 4♦ / 4♥ | Texas transfer to hearts/spades | Confirmed |
| 4NT | Quantitative, approximately 18–19 HCP | Confirmed principle; judgment may adjust the range |
| Other responses | — | TBD |

The opening may contain a five-card major and always has at least three cards
in both majors. Stayman therefore uses these answers:

- 2♦: exactly 3–3 in the majors;
- 2♥: 4+ hearts, possibly also four spades; and
- 2♠: exactly three hearts and 4+ spades.

With both four-card majors, opener shows hearts first. Responder may use 2♣
with an invitational-or-better hand containing a four-card major, a balanced
invitation without a four-card major, or a weak hand with both majors. After
any Stayman answer, 2NT is a natural invitation and 3NT is to play. After
`1NT–2♣; 2♦`, responder's 2♥ is the weak both-majors signoff. Other Stayman
continuations are **TBD**.

#### Major transfers

Opener normally completes a major transfer at 2M. The only superaccept is 3M,
showing exactly 15 HCP, four-card support, and a suitable maximum. The former
2NT support rebid and side-minor transfer breaks are not used.

After the ordinary completion, responder's continuations are:

| Continuation | Meaning |
| --- | --- |
| Pass | Weak signoff |
| 2NT | Natural invitation, normally exactly five cards in the transferred major |
| 3M | Invitational with 6+ cards in the transferred major |
| 3NT | Choice of games, normally exactly five cards in the transferred major |
| 4M | To play, normally 6+ cards in the transferred major |
| New suit | Natural GF second suit |
| 4NT | Quantitative; the transferred major has not been selected as trump |

After a Texas transfer, opener completes to 4M. Pass is to play; further action
is a slam try with the transferred major agreed as trump, so 4NT is RKCB.

#### Minor transfers

Opener completes 2♠ at 3♣ and 2NT at 3♦. Pass after completion is weak;
continuing is GF. A GF transfer may contain a second suit or shortness to be
shown later, so the minor need not be a pure one-suiter. Direct 3♣/3♦ instead
shows the invitational hand. Exact GF continuations after completion are
**TBD**.

#### Interference

After `1NT–(X)`, the normal response structure remains on and Redouble is
business-oriented. If a transfer is doubled, opener normally completes it;
Redouble is the rare business action and does not ask for completion.

After a natural suit overcall, Double is negative/cooperative, normally showing
values and interest in the unbid suits. Opener may convert it to penalty with
sufficient trump length and defensive prospects. Natural free bids require
genuine suit length. PEN-Club will use Transfer Rubensohl over suitable
two-level interference, but the exact mapping is **TBD**.

### After 2♣ — 11–15, 6+ clubs

The complete response and rebid structure is **TBD**.

### After 2♦ — 11–15, 6+ diamonds

| Response | Current meaning | Status |
| --- | --- | --- |
| 2♥ / 2♠ | Natural, 5+ cards | Draft; exact strength/force TBD |
| 2NT | Stenberg, 3+ diamond support, currently 10+ HCP | Draft |
| 3♣ | Natural GF, 5+ clubs, currently 12+ HCP | Draft; reconcile with the 25-HCP GF rule |
| 3♦ | Preemptive raise, 3+ diamonds, currently 0–9 HCP | Draft |
| Other responses | — | TBD |

After a major response, current opener rebids distinguish:

- maximum 14–15 with three-card support: 2♠ after 2♥, or 3♥ after 2♠;
- maximum 14–15 without three-card support: 2NT;
- minimum 11–13 with three-card support: 3M.

After `2♦ (X)`, redouble transfers to hearts and 2♥ transfers to spades;
opener completes the transfer. The meaning of later calls through 2NT is
**TBD**.

### After 2♥ / 2♠ — weak two

All responses, raises, feature asks, and competitive continuations are **TBD**.

### After 2NT — 22–24 balanced

All responses and continuations are **TBD**. Confirm whether the partnership
uses the same Stayman/transfer family as after 1NT, adjusted for level and
strength.

## Slam methods

### General slam search in an uncontested game force

PEN uses descriptive slow arrival when the opponents have remained silent and
the partnership is already game-forced. Before a fit is known, a hand starts
the general slam search at a known combined minimum of 29 HCP when it has useful
shape to describe:

- show the longest unshown four-card or longer side suit at the cheapest level
  below 3NT;
- otherwise rebid a previously shown six-card or longer suit below 3NT;
- prefer a new side suit to repeating the long suit;
- with no such descriptive action, place the contract normally, usually in
  3NT.

The descriptive continuation is natural, remains game forcing, and shows slam
interest. It does not set the side suit as trumps by itself. Direct arrival in
3NT instead declines this general shape-based search. The 29-point boundary is
deliberately not lowered to 28; the existing fitted-slam measurements found the
looser boundary over-aggressive.

During this search, a below-game raise explicitly establishes trumps when the
partnership's guaranteed lengths reach eight cards: two-card support opposite a
shown six, three opposite five, or four opposite four. The raise promises the
minimum support required for eight; ordinary fast-arrival game raises keep
their existing, looser support meaning.

This is a fallback agreement for unspecified continuations, not a replacement
for an authored PEN relay, splinter, ask, quantitative-notrump sequence, or
competitive treatment. It is disabled as soon as an opponent bids, doubles, or
redoubles.

### Control bidding after a fit

A non-trump suit bid is a control bid when a trump fit has been explicitly
established, the partnership is game-forced or clearly making a slam try, and
the call has no existing meaning as a relay, splinter, ask, or natural game
try. This includes rebidding a side suit that was described naturally before
trumps were set. A fit alone does not turn every new suit into a control: in a non-forcing
invitational auction, new suits remain natural or help-suit game tries.

The partnership uses mixed first- and second-round controls:

- first-round control: ace or void;
- second-round control: king or useful singleton.

A king is always a legitimate second-round control. Shortness is normally a
positive control only when partner has not shown natural length or values in
that suit; shortness in partner's suit should instead be shown through a
defined splinter or shape relay. Controls are bid economically, and skipping
an available suit normally denies both first- and second-round control there.
The initial control bid does not distinguish ace from king or void from
singleton.

Control bids need not all be made before RKCB. Direct RKCB is appropriate when
the asker knows or personally holds control of every untested side suit.
Otherwise the partnership control-bids first: RKCB counts keycards but does
not discover two immediate losers in a side suit. Returning to game instead
of making an available control bid declines further slam cooperation unless
partner's preceding call was forcing.

Notrump bids are not controls unless explicitly defined otherwise. PEN-Club
currently has no serious/non-serious 3NT agreement.

### When 4NT is quantitative

Interpret 4NT from the meaning of the auction, using this priority:

1. With an agreed trump fit, 4NT is RKCB 1430 for that suit.
2. Without an agreed fit, 4NT is quantitative when the auction is based on a
   natural notrump bid or rebid and remains notrump-oriented.
3. With neither an agreed fit nor a natural notrump anchor, 4NT is undefined
   and must not be assumed to be quantitative or RKCB.

A later trump agreement takes precedence over an earlier natural notrump bid.
Artificial 1NT/2NT relays, asks, raises, and support-showing calls are not
quantitative anchors.

| Auction | Meaning of 4NT |
| --- | --- |
| 1NT–4NT or 2NT–4NT | Quantitative |
| 1♣–1NT–4NT | Quantitative; the natural balanced positive response has not established a fit |
| 1♣–1♦; 1NT–4NT | Quantitative; opener's 1NT is natural and balanced |
| A natural 2NT or 3NT rebid followed by 4NT | Quantitative if no fit has subsequently been agreed |
| 1NT–2♦; 2♥–4NT | Quantitative: although at least eight hearts are known, responder has not selected hearts as trump and the sequence remains notrump-oriented |
| 1♦–1♥; 1NT–4NT | RKCB for hearts because the artificial 1NT rebid shows three-card support |
| 1♦–1♠; 1NT–4NT | RKCB for spades because the auction has established a spade fit |
| 1♠–2♦; 2NT–4NT | RKCB for hearts because 2NT shows the strong branch with 3+ heart support |

Quantitative 4NT is invitational and may be passed. Pass declines and 6NT
accepts; intermediate hands use judgment based on range, controls,
intermediates, and a useful five-card suit. After `1♣–1NT–4NT`, the lower
9–11 band normally declines unless exceptionally suitable, while the 14+ band
normally accepts. Natural suit-oriented acceptances beyond Pass and 6NT are
**TBD**.

### RKCB 1430

With an agreed trump suit, 4NT asks for the four aces plus the trump king:

| Response | Meaning |
| --- | --- |
| 5♣ | 1 or 4 keycards |
| 5♦ | 0 or 3 keycards |
| 5♥ | 2 or 5 keycards without the trump queen |
| 5♠ | 2 or 5 keycards with the trump queen |

The asker infers the lower or higher number from the auction and their own
cards. Known extra trump length may substitute for the trump queen only when
the partnership knows it has at least a ten-card fit.

#### Signoff and trump-queen ask

Pass signs off when responder has bid five of the agreed trump suit.
Otherwise, the cheapest available bid of the agreed trump suit is a signoff
and responder must pass. With minor trumps, a response can force the
partnership to six of the minor; the asker must anticipate this because
PEN-Club does not currently use Kickback or Redwood.

After a 5♣ or 5♦ response, the cheapest available non-trump bid asks for the
trump queen. Bidding the trump suit remains a signoff. Responses to the queen
ask are deliberately simple:

- first step: no trump queen;
- second step: trump queen.

The 5♥/5♠ initial responses already disclose the queen. No side king is shown
simultaneously with a queen response.

#### Specific-king ask

Once all keycards and the trump queen are accounted for, asker's 5NT asks for
specific side-suit kings and guarantees at least a small slam. Responder bids
the lowest side-suit king that can be shown without passing six of the agreed
trump suit, or bids six of trump with no safely showable side king. The trump
king is not shown again.

PEN-Club initially uses no second-king relay or specific-suit ask. After the
first king response, asker places the contract. A direct 5NT response to 4NT
is a void response; asker's later 5NT is the specific-king ask.

#### Useful-void responses

Responder may replace the ordinary response when holding a useful void:

| Response | Meaning |
| --- | --- |
| 5NT | An even number of keycards—0, 2, or 4—with a useful void |
| Six of a side suit below trump | An odd number—1 or 3—with a void in the bid suit |
| Six of agreed trump | An odd number—1 or 3—with a void in a higher-ranking suit |

These responses do not disclose the trump queen. To keep the continuation
simple, asker places the contract after a void response; there is no automatic
queen or king relay.

#### Exclusion RKCB

A jump to five of a non-trump suit is Exclusion RKCB when trump is
unambiguously agreed, the jump cannot reasonably be natural, and the response
space is safe. It shows a void in the bid suit and uses 1430 steps while
excluding the ace of that suit:

1. first step: 1 or 4 relevant keycards;
2. second step: 0 or 3;
3. third step: 2 without the trump queen;
4. fourth step: 2 with the trump queen.

The relevant keycards are the other three aces plus the trump king. A jump to
five of the agreed trump suit is not Exclusion. Non-jump five-level control
bids remain **TBD**.

#### Interference over RKCB

If opponents double 4NT, use ROPI:

| Call | Meaning |
| --- | --- |
| Redouble | 0 or 3 keycards |
| Pass | 1 or 4 keycards |
| First step | 2 without the trump queen |
| Second step | 2 with the trump queen |

If opponents overcall at the five-level, use DOPI:

| Call | Meaning |
| --- | --- |
| Double | 0 or 3 keycards |
| Pass | 1 or 4 keycards |
| Cheapest bid | 2 without the trump queen |
| Next bid | 2 with the trump queen |

After interference at the six-level, use DEPO: Double shows an even number of
keycards and Pass an odd number. After interference, asker normally places the
contract; void responses and later queen or king asks are abandoned unless
ample space clearly remains.

## Competitive and defensive bidding

Confirmed general policy: where no treatment is specified, calls are natural.

### After the opponents open naturally

**Confirmed foundation:** use natural overcalls and takeout Doubles. Direct-seat
strengths are approximate; suit quality, shape, vulnerability, and playing
strength may justify deviation.

| Call over a natural one-level opening | Meaning |
| --- | --- |
| Double | Takeout, normally opening values with support for the unbid suits, especially unbid majors; may instead be any 17+ raw-HCP hand |
| One-level suit overcall | Natural, normally 5+ cards and approximately 8–17 HCP |
| Two-level suit overcall | Natural, approximately 10–17 HCP; normally 6+ cards or a good five-card suit |
| 1NT | 15–18 HCP, balanced, with a stopper in opener's suit |
| Jump suit overcall | Weak and preemptive, normally a six-card suit; uses the opening-preempt HCP, vulnerability, and suit-quality gate |
| Direct cue-bid | Michaels |
| 2NT | Unusual 2NT: the two lowest unbid suits |

With 17+ raw HCP and a strong one-suited hand, Double first and then bid the
suit. A direct natural overcall is therefore limited.

Over a natural minor, an ordinary 12–16 HCP takeout Double shows at least 3–3
in the majors, at least one four-card major, and no five-card major. With a
five-card major, overcall it naturally instead. The separate 17+ any-shape
Double is measured in raw HCP; distribution does not promote a weaker hand
into that branch.

The Michaels and Unusual 2NT meanings are:

- a cue-bid of a natural minor shows both majors, 5–5 or longer;
- a cue-bid of a natural major shows the other major and an unspecified minor,
  5–5 or longer;
- 2NT shows the two lowest unbid suits, normally 5–5 or longer; and
- after a Michaels cue-bid of a major, advancer's 2NT asks for the minor.

These two-suited actions are either weak/preemptive or strong. With an
intermediate-strength hand, overcall the better suit naturally and introduce
the second suit later when appropriate.

#### Advancing a natural overcall

| Advancer's action | Meaning |
| --- | --- |
| Simple raise | Constructive, normally 3+ support |
| Jump raise | Preemptive, normally 4+ support |
| Cue-bid | INV+ with support |
| Non-jump new suit | Natural and forcing one round by an unpassed hand; natural and nonforcing by a passed hand |
| Jump in a new suit | Natural and invitational with a good 6+ suit |
| Notrump | Natural, with a stopper in opener's suit |

After the opponents bid again, advancer's Double is negative/cooperative unless
an explicit auction-specific meaning applies.

#### After our takeout Double

| Advancer's action | Meaning |
| --- | --- |
| Pass | Penalty conversion, normally at least five good cards in opener's suit and adequate defensive strength |
| Cheapest suit response | Approximately 0–8 HCP |
| Jump in a suit | Approximately 9–11 HCP |
| Cue-bid | Strong and forcing; the exact game-force threshold is TBD |
| 1NT | Approximately 6–10 HCP with a stopper |
| 2NT | Approximately 11–12 HCP with a stopper |
| 3NT | To play |

If the opponents raise, Double by advancer is responsive. Doubler's new suit
after a minimum response shows the strong-hand type, normally about 18+ HCP.

#### Balancing seat

- Natural suit bids and takeout Doubles may be about a king lighter than in
  direct seat.
- A balancing 1NT shows approximately 11–14 HCP, balanced; normal notrump
  methods apply.
- A balancing jump is natural and constructive rather than weak.

#### Against preempts

- Suit overcalls are natural.
- A direct Double is takeout-oriented through at least 4♥. Over 4♠ it is more
  defensive/cooperative. This is an explicit exception to the normal
  game-level penalty-Double rule.
- 2NT over a weak two shows approximately 15–18 HCP, balanced, with a stopper;
  normal notrump methods apply.
- A Transfer Lebensohl structure after partner Doubles a weak two may be added
  later; its mapping is **TBD**.

#### Against a natural 1NT opening

Use a simple Landy-plus-natural structure:

| Call | Meaning |
| --- | --- |
| Double | Penalty/values; required strength is adjusted to the opponents' notrump range |
| 2♣ | Both majors, normally at least 5–4 |
| 2♦ / 2♥ / 2♠ | Natural, normally 5+ cards |
| 2NT | Both minors, normally 5–5 or longer |

#### Against an artificial strong 1♣ opening

Use a simple natural structure with two Mathe-style two-suiters. The two
conventional calls are swapped from original Mathe so that 1NT shows the
requested major-suit hand:

| Call | Meaning |
| --- | --- |
| Double | Both minors, 5–5 or longer; weak or strong |
| 1♦ / 1♥ / 1♠ | Natural, normally 5+ cards |
| 1NT | Both majors, 5–5 or longer; weak or strong |
| 2♣ | Natural, normally 6+ clubs |
| 2♦ / 2♥ / 2♠ | Natural weak jump, normally 6+ cards |

After Double or 1NT, advancer chooses the longer advertised suit, breaking a
tie toward clubs or hearts respectively. The intervenor then normally passes.
Direct natural one-level overcalls are uncapped so a strong one-suited hand is
not forced into a Double whose meaning is two-suited.

Other artificial and multi-meaning openings require separate defenses. No
general meaning is assigned because the bid suit may be natural, may show
another suit, or may show no suit at all.

### When our artificial or forcing call is doubled

The default is to retain the undoubled system meanings. A transfer is completed
by bidding its destination; Redouble does not generically ask partner to
complete it.

| Situation | Pass | Redouble | Completion or normal answer | New suit |
| --- | --- | --- | --- | --- |
| Transfer doubled | Normally unavailable | Business: a genuine holding in the artificial suit and willingness to play there | Retains its undoubled meaning | Retains the normal transfer-break meaning |
| Relay or ask doubled | Normally unavailable while an answer is required | Business or penalty interest | Answer normally, ignoring the double | Only if already a defined relay answer |
| Natural forcing bid doubled | Forcing, with no clear descriptive action | Extra strength and willingness to defend or play doubled | A raise remains natural | Natural and retains its force |
| Artificial opening doubled | Auction-specific | Auction-specific | Existing system remains on where defined | Retains its existing meaning |

A business Redouble over an artificial call is rare and must be safe if
everyone passes. Auction-specific treatments override the default. Confirmed
examples include `1♠–(X)–XX` as the INV+ general ask and the replacement
transfer Doubles after natural 2♦/2♥ overcalls of 1♠. The current
`2♦–(X)–XX` transfer to hearts remains Draft until that opening is reviewed.

The current authored application is deliberately general rather than a list of
hand-by-hand rescues:

- after doubled 1♣, 1♦, and 1♥ artificial openings, the normal response
  structure remains on; after a doubled negative response or relay, opener
  still makes the normal required rebid;
- after a doubled natural 2♣, 2♥, or 2♠ opening, responder may Redouble with
  genuine business values, raise with support, or show a five-card alternative;
  if responder Passes and fourth hand Passes, opener runs only with a second
  five-card suit;
- after a doubled natural overcall, normal advances remain available, a sound
  Redouble is business, and a weak advancer may show a five-card alternative;
  after Pass–Pass, overcaller runs only with a second five-card suit;
- after doubled Landy, Michaels, or Unusual 2NT, advancer chooses an advertised
  suit (or uses the defined Michaels minor ask); and
- a doubled splinter or control bid remains forcing until the partnership has
  reached game in the known trump suit. It may not become the final contract by
  accident.

In an unauthored competitive continuation, the natural fallback may bid four
of a major only with a known eight-card fit and at least 25 combined raw HCP,
or with a known nine-card fit when non-vulnerable against vulnerable opponents.
An authored game force is exempt from this conservative gate.

### General low-level Double policy

**Confirmed umbrella rule:** low-level Doubles are takeout or cooperative and
may be converted to penalty by passing with sufficient trump length. The
following priority determines the meaning:

1. An explicitly defined conventional Double takes precedence. This includes
   replacement transfers and the general-ask Double after `1♠–(2♠)`.
2. An explicitly agreed penalty-oriented Double applies, such as Double of a
   natural 1NT overcall or the cooperative penalty Double after a natural 2♣
   overcall of 1♠.
3. Before our side has established a fit, responder's first Double is normally
   negative/takeout: values and tolerance for the unbid suits, not pure
   penalty.
4. After partner has made a takeout Double and the opponents raise, Double is
   responsive unless explicitly defined otherwise.
5. After our side has found a fit, or when both sides are competing with known
   fits, Double is cooperative: it shows defensive values and no clear
   offensive action. Partner passes with appropriate trumps and defensive
   prospects, but pulls with offensive shape or inadequate trumps.
6. Game-level Doubles are normally penalty unless the auction clearly demands
   takeout.

Exact strength, trump-length, vulnerability, and level requirements for a
penalty conversion are **TBD**.

The following competitive areas still need explicit agreements:

- interference above the one-level over 1♣, and responses and rebids after
  interference over 1♦ and 1♥;
- interference over an artificial ask or transfer after responder has acted;
- detailed continuations after overcalls, takeout Doubles, and preemptive
  openings;
- defenses to artificial and multi-meaning openings other than a strong 1♣;
- vulnerability- and position-dependent adjustments outside the confirmed
  preempt gate.

## Questions to complete next

1. Tie-break for equal positive majors after 1♣ and equal transfer majors.
2. Exact distributional evaluation after a fit is known.
3. Exact INV bands for each concrete opening and response.
4. Meanings of 1♦–2NT and the invitational jump responses.
5. Meanings of 1♦–4♠ and 1♥–3♠/4♠.
6. The 1♣ positive-major Marmic singleton step mapping.
7. Detailed relay continuations after the artificial 1♠ asks, including the
   competitive Rubensohl GF relays.
8. Responder's constructive and GF continuations after the 1♠ major-transfer
   rebids, including later interference.
9. Complete response structure after 2♣, both weak twos, and 2NT.
10. Exact mapping after `2♦ (X)` beyond redouble and 2♥.
11. Competitive structure after opponents disturb 1♥.
12. Exact penalty-double and penalty-conversion requirements, including
    conversion of a negative/cooperative Double after interference over 1NT.
13. The running-major 3NT opening.
14. Natural suit-oriented acceptances of quantitative 4NT, trump selection
    when more than one fit is agreed, and non-jump five-level controls.
15. Defenses to artificial and multi-meaning openings, plus the exact Transfer
    Lebensohl structure after a takeout Double of a weak two.
16. Direct 3♥/3♠ responses to 1NT, later Stayman and transfer continuations,
    and the exact Transfer Rubensohl mapping over 1NT interference.
