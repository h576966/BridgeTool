# BridgeTool project

## Purpose

BridgeTool is a personal hobby project for a single developer. Its long-term
purpose is to define custom bridge bidding systems, select separate systems for
NS and EW, inspect coverage, overlap, and bidding frequencies, practise against
robots, and compare systems or conventions through reproducible experiments.
DDS and BBA/EPBot provide analysis and reference baselines.

The near-term priority is a stable, understandable local foundation with low
maintenance cost. It is not yet the implementation of the custom bidding
system.

## Why Pons

[Pons](https://github.com/jdh8/pons) already provides the bridge-specific
engine, inference model, analysis tools, tests, experiment harnesses, and web
application that BridgeTool needs. Reusing it avoids maintaining a second
engine and gives the project a tested baseline. Pons is therefore imported with
its Git history intact and remains the core crate and engine.

## Main components

- **Pons bidding engine:** authored bidding books, deterministic and learned
  fallback policies, auction handling, and the existing American system.
- **Constraints and inference:** machine-readable hand constraints and the
  meanings that partner and opponents may infer from calls.
- **DDS:** native double-dummy analysis used by tests, examples, and scoring.
- **BBA/EPBot adapter:** the pinned `vendor/bba` submodule and Pons examples for
  comparison with an external reference bidder.
- **WebAssembly web app:** the existing local, client-side Pons interface for
  practice, demos, book inspection, deal editing, separate NS/EW profiles, and
  read-only inspection of the provisional BridgeTool opening audit.
- **Future BridgeTool system module:** the custom system, added only after its
  meanings and priorities are precise enough to implement and test.
- **Future analysis and experiment tools:** coverage, overlap, frequency, and
  reproducible paired A/B reports, preferably extending existing Pons tools.

## Upstream synchronization

`origin` points to BridgeTool and `upstream` points to Pons. Fetch future Pons
changes with `git fetch upstream`, inspect them, and merge `upstream/main` into
BridgeTool's `main` without rewriting published history. Keep BridgeTool changes
small and isolated so conflicts remain easy to review. Preserve upstream
attribution, the Apache-2.0 license, and the BBA submodule.

Git remotes are local configuration and are not versioned. Add Pons in each new
clone with `git remote add upstream https://github.com/jdh8/pons.git`, then
confirm the configuration with `git remote -v`.

## Stages

1. **Stage 0:** reproducible baseline and development environment.
2. **Stage 1:** custom opening system plus coverage and frequency analysis.
3. **Stage 2:** uncontested responses and rebids, one opening family at a time.
4. **Stage 3:** competitive bidding and defenses.
5. **Stage 4:** paired A/B experiments against the standard system and
   BBA/EPBot.
6. **Stage 5:** improved local web app and separate NS/EW profiles.
7. **Stage 6:** possible data file or DSL for bidding systems, only if
   system-as-code proves impractical.

## Near-term non-goals

- A commercial product.
- User accounts or cloud services.
- Multiplayer.
- Mobile applications.
- A custom advanced card-play robot.
- ML training.
- A general graphical bidding-system editor.
- A broad redesign of the Pons web interface.
