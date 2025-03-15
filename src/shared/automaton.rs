use crate::automatons::{dfa::DFA, nfa::NFA, pda::PDA};

// The Place with all the Boilerplate

// AutomatonData

pub type VertexId = u32;

pub enum AutomatonData {
    Edge(VertexId, VertexId, String),
    Final(VertexId),
    Start(VertexId),
}

// AutomatonType

#[allow(clippy::upper_case_acronyms)]
pub enum AutomatonType {
    DFA,
    NFA,
    PDA,
}

pub fn determine_automaton_type(typestr: &str) -> AutomatonType {
    match typestr.to_lowercase().as_str() {
        "dfa" => AutomatonType::DFA,
        "nfa" => AutomatonType::NFA,
        "pda" => AutomatonType::PDA,
        _ => unimplemented!("type {} is not supported", typestr),
    }
}

pub fn path_to_automaton_type(filepath: &str) -> String {
    vec!["dfa", "nfa", "pda"]
        .into_iter()
        .find(|pattern| filepath.to_lowercase().contains(pattern))
        .expect("No Automaton Type could be determined")
        .to_string()
}

// The Automaton Interface Enum

// Macro to generate the Implementation

macro_rules! gen_impl {
    ($enum:ident, $function:ident, $return:ty, $($arg:ident; $atype:ty),*) => {
        impl $enum {
            pub fn $function(&self, $($arg: $atype),*) -> $return {
                match self {
                    $enum::DFA(a) => a.$function($($arg),*),
                    $enum::NFA(a) => a.$function($($arg),*),
                    $enum::PDA(a) => a.$function($($arg),*),
                }
            }
        }
    };
}

#[allow(clippy::upper_case_acronyms)]
pub enum Automaton {
    DFA(DFA),
    NFA(NFA),
    PDA(PDA),
}

gen_impl!(Automaton, accepts, bool, word; &str);
gen_impl!(Automaton, alphabet, &Vec<char>,);
gen_impl!(Automaton, view, (),);
