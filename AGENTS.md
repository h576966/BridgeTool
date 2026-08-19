# BridgeTool agent instructions

## Project and working style

- This is a private hobby project for one developer. Prefer the simplest
  solution that meets the current need.
- Avoid enterprise architecture, unnecessary abstractions, placeholder
  modules, and new dependencies without a concrete need.
- Make small, cohesive changes that are easy to review and revert. Do not
  change unrelated upstream code.
- Preserve the ability to sync future changes from Pons through the `upstream`
  remote. Keep Pons as the core engine and crate name until there is a concrete
  reason to change it.
- Record durable decisions in the repository, not only in chat.
- The user is the authority on bidding-system meanings. Do not silently resolve
  bridge ambiguities; document them as open questions.

## Git

- Work directly on `main` unless the user explicitly requests another branch.
- Do not open pull requests or create feature branches unless explicitly asked.
- Use small, descriptive commits. Never rewrite published `main` history.
- Before committing, review the complete diff, run relevant checks, and run
  `git diff --check`.
- Commit and push only when the current prompt explicitly authorizes it.
- Natural milestones will often use a separate review followed by commit and
  push.

## Bidding code and experiments

- Read `docs/bidding-architecture.md` before changing `src/bidding`.
- Read `docs/measurement.md` before A/B measurements or claims about system
  strength.
- Every artificial call needs an alert, a machine-readable meaning/inference,
  and tests.
- Keep separate the rule selecting a call for the actual hand and the meaning
  that partner and opponents are allowed to read.
- Bidding changes must be comparable with an unchanged baseline.
- Reproducible experiments must record the seed, system version, and settings.
- Do not run large A/B campaigns or ML training unless explicitly requested.

## Verification

Run the relevant Pons checks before delivery:

```text
cargo fmt --check
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features
```

Run `cd web && cargo test` when the web module or public API changes. If an
upstream check already fails, distinguish an environment problem from a code
failure, document it, and do not make unrelated changes to hide it.
