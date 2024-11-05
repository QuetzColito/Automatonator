mod automatons;

use automatons::automaton::*;

use crate::automatons::parsing::*;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Automaton to test
    automaton: String,

    /// The Reference Automaton to test against
    automaton2: Option<String>,

    /// The Automaton Type (dfa, nfa, pda, tm)
    #[arg(short = 't', long = "type")]
    automaton_type: Option<String>,

    /// The Automaton Type of the Reference Automaton (in case it differs from main type)
    #[arg(short = 'r', long = "reftype")]
    ref_automaton_type: Option<String>,

    /// Path to a File with words to check (line format: "(0|1),word")
    #[arg(short = 'c', long = "checks")]
    testcase_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let filepath = &args.automaton;
    println!("Reading Automaton from {}", filepath);
    let file = fs::read_to_string(filepath).expect("file doesn't exist");

    let automaton_type = determine_automaton_type(
        &args
            .automaton_type
            .unwrap_or(path_to_automaton_type(&filepath)),
    );

    let automat = parse_automaton(file, automaton_type, filepath.ends_with(".xml"));
    println!("Successfully read Automaton:");

    automat.view();
    assert!(automat.accepts("aaa"), "Did not accept aaa");
    assert!(!automat.accepts("aaaa"), "Did accept aaaa");
}
