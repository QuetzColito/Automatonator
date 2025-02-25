use std::collections::HashSet;

use super::dfa::DFA;
use super::nfa::NFA;
use super::pda::PDA;

// The Place with all the Boilerplate

pub type VertexId = u32;

pub fn format_states(states: &HashSet<VertexId>) -> String {
    states
        .iter()
        .map(|id| id.to_string())
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}

pub enum AutomatonData {
    Edge(VertexId, VertexId, String),
    Final(VertexId),
    Start(VertexId),
}

pub enum Automaton {
    DFA(DFA),
    NFA(NFA),
    PDA(PDA),
}

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

impl Automaton {
    pub fn accepts(&self, word: &str) -> bool {
        match self {
            Automaton::DFA(dfa) => dfa.accepts(word),
            Automaton::NFA(nfa) => nfa.accepts(word),
            Automaton::PDA(pda) => pda.accepts(word),
        }
    }

    pub fn alphabet(&self) -> &Vec<char> {
        match self {
            Automaton::DFA(dfa) => dfa.alphabet(),
            Automaton::NFA(nfa) => nfa.alphabet(),
            Automaton::PDA(pda) => pda.alphabet(),
        }
    }

    pub fn view(&self) {
        match self {
            Automaton::DFA(dfa) => dfa.view(),
            Automaton::NFA(nfa) => nfa.view(),
            Automaton::PDA(pda) => pda.view(),
        }
    }
}
