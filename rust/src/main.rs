//! CLI for the DED symbiosis inverter. Takes a CSV path and prints the boxed
//! Base-38 inversion telemetry. With no argument it runs the built-in mock
//! candidates.

use ded_ark_engine::{parse_candidates_csv, run_inversion_audit, run_inversion_audit_for};
use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "usage: ded_ark_cli [--help] [--version] [PATH]";

fn print_help() {
    print!(
        r#"DED symbiosis inverter — theoretical computational model

{USAGE}

With no PATH, runs the built-in synthetic candidates.
PATH is a CSV with columns:
  organism, stage, continuous_score, target_frequency_hz

Options:
  -h, --help       show this help
  -V, --version    show version

No clinical or biological efficacy claim.
"#
    );
}

fn print_footer() {
    println!("\nnote: theoretical computational model; no clinical or biological efficacy claim.");
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let arg = args.next();
    if args.next().is_some() {
        eprintln!("error: expected at most one PATH argument");
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    }

    match arg.as_deref() {
        None => {
            run_inversion_audit();
            print_footer();
            ExitCode::SUCCESS
        }
        Some("-h") | Some("--help") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("-V") | Some("--version") => {
            println!("ded_ark_cli {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(s) if s.starts_with('-') => {
            eprintln!("error: unknown option: {s}");
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
        Some(path) => match parse_candidates_csv(Path::new(path)) {
            Ok(candidates) if !candidates.is_empty() => {
                run_inversion_audit_for(&candidates);
                print_footer();
                ExitCode::SUCCESS
            }
            Ok(_) => {
                eprintln!("error: no candidates parsed from {path}");
                ExitCode::FAILURE
            }
            Err(e) => {
                eprintln!("error: {e}");
                eprintln!("{USAGE}");
                ExitCode::FAILURE
            }
        },
    }
}
