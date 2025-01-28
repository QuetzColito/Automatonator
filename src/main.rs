mod args;
mod automatons;

use std::fs;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use args::Args;
use automatons::evaluation::*;
use automatons::parsing::*;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let mut rng = ChaCha8Rng::seed_from_u64(43);
    println!("{}", rng.gen_range(0..100));
    println!("{}", rng.gen_range(0..100));
    println!("{}", rng.gen_range(0..100));
    println!("{}", rng.gen_range(0..100));
    println!("Reading Automaton from {}", &args.automaton);

    let automat = parse_automaton(args.automaton, &args.automaton_type);
    println!("Successfully read Automaton:");
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

        // Evaluate if evaluation_file given
        if let Some(evaluation_file) = args.evaluation_file {
            let cases = fs::read_to_string(&evaluation_file).expect("file doesn't exist");

            println!(
                "Automatons answered the same on {}% of",
                full_comparison(&automat, &automat2, &cases)
            );
        }
    }

    // Compare to Test Cases (if given)
    if let Some(testcase_filepath) = args.testcase_file {
        let cases = fs::read_to_string(&testcase_filepath).expect("file doesn't exist");
        fixed_test(&automat, &cases);
    }
}
