mod automatons;

use automatons::automaton::AutomatonType;

use crate::automatons::parsing::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("No Automaton given");
    } else {
        let filepath = args[1].clone();
        println!("Reading Automaton from {}", filepath);
        let file = fs::read_to_string(filepath.clone()).expect("file doesn't exist");

        let automat = parse_automaton(file, AutomatonType::DFA, filepath.ends_with(".xml"));
        println!("Successfully read Automaton:");

        automat.view();
        assert!(automat.accepts("aaa"), "Did not accept aaa");
        assert!(!automat.accepts("aaaa"), "Did accept aaaa");
    }
}
