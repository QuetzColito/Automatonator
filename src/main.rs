mod args;
mod automatons;
mod shared;

use log::*;
use std::fs;

use args::Args;
use clap::Parser;
use shared::evaluation::*;
use shared::parsing::*;

fn main() {
    let args = Args::parse();
    colog::init();

    info!("Reading Automaton from {}", &args.automaton);

    let automat = parse_automaton(args.automaton, &args.automaton_type);
    info!("Successfully read Automaton:");
    automat.view();

    // Compare to Reference Automaton (if given)
    if let Some(filepath2) = args.automaton2 {
        let automat2 = parse_automaton(
            filepath2,
            if args.ref_automaton_type.is_some() {
                &args.ref_automaton_type
            } else {
                &args.automaton_type
            },
        );

        info!("Successfully read second Automaton:");
        automat2.view();

        // Evaluate if evaluation_file given
        if let Some(evaluation_file) = args.evaluation_file {
            let cases = fs::read_to_string(&evaluation_file).expect("file doesn't exist");

            println!(
                "Automatons answered the same on {}% of",
                full_comparison(&automat, &automat2, &cases)
            );
        } else if generated_comparison(&automat, &automat2) == 1 {
            info!("passed generated comparison")
        } else {
            warn!("did not pass generated comparison")
        }
    }

    // Compare to Test Cases (if given)
    if let Some(testcase_filepath) = args.testcase_file {
        let cases = fs::read_to_string(&testcase_filepath).expect("file doesn't exist");
        fixed_test(&automat, &cases);
    }
}
