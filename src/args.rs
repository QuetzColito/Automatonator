use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// The Automaton to test
    pub automaton: String,

    /// The Reference Automaton to test against
    pub automaton2: Option<String>,

    /// The Automaton Type (dfa, nfa, pda, tm)
    #[arg(short = 't', long = "type")]
    pub automaton_type: Option<String>,

    /// The Automaton Type of the Reference Automaton (in case it differs from main type)
    #[arg(short = 'r', long = "reftype")]
    pub ref_automaton_type: Option<String>,

    /// Path to a File with words to check (line format: "(0|1),word")
    #[arg(short = 'c', long = "checks")]
    pub testcase_file: Option<String>,
}