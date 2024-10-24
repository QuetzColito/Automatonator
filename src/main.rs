mod automatons;

use crate::automatons::shared::*;
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

        let automat = if filepath.ends_with(".xml") {
            parse_xml_automaton(file)
        } else {
            parse_text_automaton(file)
        };

        println!("{}", automat.accepts("aaa"));
        println!("{}", automat.accepts("aaaaaa"));
        println!("{}", automat.accepts("aaaaaaaaaa"));
    }
}
