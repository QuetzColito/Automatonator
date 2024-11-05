mod args;
mod automatons;

use args::Args;
use automatons::automaton::*;
use automatons::parsing::*;
use clap::Parser;

use std::fs;

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
