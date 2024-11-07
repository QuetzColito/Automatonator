mod args;
mod automatons;

use std::fs;

use args::Args;
use automatons::parsing::*;
use clap::Parser;

fn main() {
    let args = Args::parse();

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

        // TODO: generate test words
        let acceptance_ratio = vec!["aaa", "aaaaa", "aa"]
            .iter()
            .filter(|word| automat2.accepts(word) == automat.accepts(word))
            .count()
            / 2;
        println!(
            "Automatons answered the same on {}% of",
            acceptance_ratio * 100
        );
    }

    // Compare to Test Cases (if given)
    if let Some(testcase_filepath) = args.testcase_file {
        let cases = fs::read_to_string(&testcase_filepath).expect("file doesn't exist");
        let cases = cases.lines();

        let count = cases.clone().count();

        let acceptance_ratio = cases.filter(|word| automat.accepts(word)).count() / count;

        println!(
            "Automatons answered the same on {}% of",
            acceptance_ratio * 100
        );
    }
}
