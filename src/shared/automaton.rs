use crate::automatons::{dfa::DFA, nfa::NFA, pda::PDA};

// The Place with all the Boilerplate

pub type VertexId = u32;

pub enum AutomatonData {
    Edge(VertexId, VertexId, String),
    Final(VertexId),
    Start(VertexId),
}

#[allow(clippy::upper_case_acronyms)]
pub enum Automaton {
    DFA(DFA),
    NFA(NFA),
    PDA(PDA),
}

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
