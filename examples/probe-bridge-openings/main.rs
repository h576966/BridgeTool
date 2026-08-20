//! Reproducible census of the provisional BridgeTool opening rules.

use std::collections::BTreeMap;
use std::env;
use std::process::ExitCode;

use contract_bridge::deck::full_deal;
use contract_bridge::{Hand, Seat};
use pons::bidding::bridge_tool::{
    HandFacts, Opening, OpeningSelection, eligible_openings, minor_exception_candidates,
    select_opening,
};
use rand::SeedableRng;
use rand::rngs::StdRng;

const AUDIT_VERSION: &str = "bridge-tool-opening-audit-v1";
const DEFAULT_COUNT: usize = 100_000;
const DEFAULT_SEED: u64 = 20_260_820;
const SAMPLE_LIMIT: usize = 3;

#[derive(Clone, Copy)]
struct Args {
    count: usize,
    seed: u64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            count: DEFAULT_COUNT,
            seed: DEFAULT_SEED,
        }
    }
}

#[derive(Default)]
struct Bucket {
    count: usize,
    samples: Vec<Hand>,
}

impl Bucket {
    fn record(&mut self, hand: Hand) {
        self.count += 1;
        if self.samples.len() < SAMPLE_LIMIT {
            self.samples.push(hand);
        }
    }
}

#[derive(Default)]
struct Results {
    eligible: BTreeMap<Opening, usize>,
    selected: BTreeMap<Opening, usize>,
    no_match: Bucket,
    ambiguous: usize,
    overlaps: BTreeMap<Vec<Opening>, Bucket>,
    minor_exceptions: BTreeMap<Opening, Bucket>,
}

impl Results {
    fn record(&mut self, hand: Hand) {
        let eligible = eligible_openings(hand);
        for &opening in &eligible {
            *self.eligible.entry(opening).or_default() += 1;
        }
        if eligible.len() > 1 {
            self.overlaps.entry(eligible).or_default().record(hand);
        }

        match select_opening(hand) {
            OpeningSelection::Selected(opening) => {
                *self.selected.entry(opening).or_default() += 1;
            }
            OpeningSelection::NoMatch => self.no_match.record(hand),
            OpeningSelection::Ambiguous(_) => self.ambiguous += 1,
        }

        for opening in minor_exception_candidates(hand) {
            self.minor_exceptions
                .entry(opening)
                .or_default()
                .record(hand);
        }
    }
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(Some(args)) => args,
        Ok(None) => return ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!();
            print_usage();
            return ExitCode::FAILURE;
        }
    };

    let results = run(args);
    print_results(args, &results);
    ExitCode::SUCCESS
}

fn parse_args() -> Result<Option<Args>, String> {
    let mut parsed = Args::default();
    let mut args = env::args().skip(1);

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--count" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value after --count".to_owned())?;
                parsed.count = value
                    .parse()
                    .map_err(|_| format!("invalid hand count: {value}"))?;
            }
            "--seed" => {
                let value = args
                    .next()
                    .ok_or_else(|| "missing value after --seed".to_owned())?;
                parsed.seed = value
                    .parse()
                    .map_err(|_| format!("invalid seed: {value}"))?;
            }
            "-h" | "--help" => {
                print_usage();
                return Ok(None);
            }
            _ => return Err(format!("unknown argument: {flag}")),
        }
    }

    if parsed.count == 0 {
        return Err("--count must be greater than zero".to_owned());
    }
    Ok(Some(parsed))
}

fn print_usage() {
    println!(
        "Usage: cargo run --no-default-features --example probe-bridge-openings -- [--seed N] [--count N]"
    );
    println!("Defaults: --seed {DEFAULT_SEED} --count {DEFAULT_COUNT}");
}

fn run(args: Args) -> Results {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let mut results = Results::default();
    let mut seen = 0usize;

    'deals: loop {
        let deal = full_deal(&mut rng);
        for seat in Seat::ALL {
            results.record(deal[seat]);
            seen += 1;
            if seen == args.count {
                break 'deals;
            }
        }
    }
    results
}

fn print_results(args: Args, results: &Results) {
    println!("BridgeTool provisional opening audit");
    println!("version: {AUDIT_VERSION}");
    println!("seed: {}", args.seed);
    println!("hands: {}", args.count);
    println!("sample limit per category: {SAMPLE_LIMIT}");

    println!();
    println!("Eligible openings");
    println!("{:<8} {:>10} {:>10}", "opening", "count", "percent");
    for opening in Opening::ALL {
        let count = results.eligible.get(&opening).copied().unwrap_or(0);
        println!(
            "{:<8} {:>10} {:>9.3}%",
            opening,
            count,
            percentage(count, args.count)
        );
    }

    println!();
    println!("Selected openings");
    println!("{:<8} {:>10} {:>10}", "opening", "count", "percent");
    for opening in Opening::ALL {
        let count = results.selected.get(&opening).copied().unwrap_or(0);
        println!(
            "{:<8} {:>10} {:>9.3}%",
            opening,
            count,
            percentage(count, args.count)
        );
    }

    println!();
    println!(
        "No match: {} ({:.3}%)",
        results.no_match.count,
        percentage(results.no_match.count, args.count)
    );
    print_samples(&results.no_match.samples);
    println!(
        "Ambiguous after explicit priorities: {} ({:.3}%)",
        results.ambiguous,
        percentage(results.ambiguous, args.count)
    );

    println!();
    println!("Opening overlaps");
    if results.overlaps.is_empty() {
        println!("  none");
    }
    for (openings, bucket) in &results.overlaps {
        let resolution = if openings.contains(&Opening::OneNotrump) {
            "selected 1NT"
        } else {
            "unresolved"
        };
        println!(
            "  {}: {} ({:.3}%, {resolution})",
            opening_list(openings),
            bucket.count,
            percentage(bucket.count, args.count)
        );
        print_samples(&bucket.samples);
    }

    println!();
    println!("Possible 6–4 minor exceptions (diagnostic only)");
    for opening in [Opening::TwoClubs, Opening::TwoDiamonds] {
        let empty = Bucket::default();
        let bucket = results.minor_exceptions.get(&opening).unwrap_or(&empty);
        println!(
            "  {opening}: {} ({:.3}%)",
            bucket.count,
            percentage(bucket.count, args.count)
        );
        print_samples(&bucket.samples);
    }
}

fn percentage(count: usize, total: usize) -> f64 {
    100.0 * count as f64 / total as f64
}

fn opening_list(openings: &[Opening]) -> String {
    openings
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" + ")
}

fn print_samples(samples: &[Hand]) {
    for &hand in samples {
        println!("    {hand} — {}", HandFacts::from(hand));
    }
}
