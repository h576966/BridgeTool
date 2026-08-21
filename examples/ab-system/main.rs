//! Generate a head-to-head A/B dump for two complete Pons bidding systems.
//!
//! At table A the treatment sits North/South against the baseline; at table B
//! the pairs swap. Both partnerships read the opponents from the system they
//! actually face. Positive IMPs favor the treatment.
//!
//! ```text
//! cargo run --release --no-default-features --features serde --example ab-system -- \
//!   --count 2000 --seed 20260820 --output ab-results/system-none.json \
//!   --treatment pen-club --baseline american --vulnerability none
//! target/release/examples/bba-score ab-results/system-none.json
//! target/release/examples/bba-score ab-results/system-none.json --score pd
//! ```

use clap::Parser;
use contract_bridge::auction::Auction;
use contract_bridge::deck::full_deal;
use contract_bridge::{AbsoluteVulnerability, Seat};
use pons::bidding::agreements::Agreements;
use pons::bidding::{Partnership, System, Table};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rayon::prelude::*;

#[derive(serde::Serialize)]
struct Board {
    deal: contract_bridge::FullDeal,
    dealer: Seat,
    table_a: Auction,
    table_b: Auction,
}

#[derive(serde::Serialize)]
struct Dump {
    generator: &'static str,
    crate_version: &'static str,
    system_specification: &'static str,
    our_label: String,
    their_label: String,
    vulnerability: AbsoluteVulnerability,
    seed: Option<u64>,
    gen_args: Vec<String>,
    boards: Vec<Board>,
}

#[derive(Parser)]
#[command(about = "Compare two complete Pons systems in a seeded seat-swap match")]
struct Args {
    /// Number of boards (dealer rotates by board)
    #[arg(short, long, default_value = "2000")]
    count: usize,

    /// First deal seed; board i uses seed + i
    #[arg(long, default_value = "20260820")]
    seed: u64,

    /// Write the JSON dump here; default is stdout
    #[arg(short, long)]
    output: Option<String>,

    /// Measured system: american, american-instinct, or pen-club
    #[arg(long, default_value = "pen-club")]
    treatment: String,

    /// Control system: american, american-instinct, or pen-club
    #[arg(long, default_value = "american")]
    baseline: String,

    /// Vulnerability: none, ns, ew, both. Use none or both for a fair seat swap.
    #[arg(short, long, default_value = "none")]
    vulnerability: AbsoluteVulnerability,
}

fn system(name: &str, agreements: &Agreements) -> anyhow::Result<System> {
    Ok(match name {
        "american" => pons::american(agreements),
        "american-instinct" => pons::american_instinct(agreements),
        "pen-club" => pons::pen_club(agreements),
        other => anyhow::bail!("system must be american|american-instinct|pen-club, got {other:?}"),
    })
}

fn declared_pair(ours: &System, theirs: &System) -> (Partnership, Partnership) {
    let ours = ours.bind();
    let theirs = theirs.bind();
    (
        ours.clone().with_opponents(&theirs),
        theirs.with_opponents(&ours),
    )
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    anyhow::ensure!(args.count > 0, "--count must be positive");
    anyhow::ensure!(
        matches!(
            args.vulnerability,
            AbsoluteVulnerability::NONE | AbsoluteVulnerability::ALL
        ),
        "use vulnerability none or both: ns/ew would give the swapped pairs different vulnerability"
    );

    let agreements = Agreements::default();
    let treatment = system(&args.treatment, &agreements)?;
    let baseline = system(&args.baseline, &agreements)?;
    let (treatment, baseline) = declared_pair(&treatment, &baseline);
    let deals = (0..args.count)
        .map(|index| {
            full_deal(&mut StdRng::seed_from_u64(
                args.seed.wrapping_add(index as u64),
            ))
        })
        .collect::<Vec<_>>();

    let boards = deals
        .into_par_iter()
        .enumerate()
        .map(|(index, deal)| {
            let dealer = Seat::ALL[index % Seat::ALL.len()];
            let treatment_ns = Table::new(
                treatment.clone(),
                baseline.clone(),
                dealer,
                args.vulnerability,
            )
            .bid_out(&deal);
            let baseline_ns = Table::new(
                baseline.clone(),
                treatment.clone(),
                dealer,
                args.vulnerability,
            )
            .bid_out(&deal);
            Board {
                deal,
                dealer,
                table_a: treatment_ns,
                table_b: baseline_ns,
            }
        })
        .collect::<Vec<_>>();

    let dump = Dump {
        generator: "examples/ab-system",
        crate_version: env!("CARGO_PKG_VERSION"),
        system_specification: "docs/pen-club-system.md",
        our_label: args.treatment,
        their_label: args.baseline,
        vulnerability: args.vulnerability,
        seed: Some(args.seed),
        gen_args: std::env::args().skip(1).collect(),
        boards,
    };
    match args.output.as_deref() {
        Some(path) => {
            serde_json::to_writer(std::io::BufWriter::new(std::fs::File::create(path)?), &dump)?
        }
        None => serde_json::to_writer(std::io::stdout().lock(), &dump)?,
    }
    eprintln!(
        "ab-system: {} (treatment) vs {} (baseline), vulnerability {} — wrote {} boards{}",
        dump.our_label,
        dump.their_label,
        dump.vulnerability,
        dump.boards.len(),
        args.output
            .as_deref()
            .map_or_else(|| " to stdout".to_owned(), |path| format!(" to {path}"))
    );

    Ok(())
}
