use std::collections::HashSet;

use super::dfa::DFA;
use super::nfa::NFA;

pub type VertexId = String;

pub fn format_states(states: &HashSet<VertexId>) -> String {
    states
        .iter()
        .map(|id| format_id(id).to_string())
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}

pub fn format_id(id: &str) -> &str {
    id.split_once('-').unwrap_or(("", &id)).1
}

pub enum AutomatonData {
    Edge(VertexId, VertexId, String),
    Final(VertexId),
    Start(VertexId),
}

pub enum Automaton {
    DFA(DFA),
    NFA(NFA),
}

pub enum AutomatonType {
    DFA,
    NFA,
}

impl Automaton {
    pub fn accepts(&self, word: &str) -> bool {
        match self {
            Automaton::DFA(dfa) => dfa.accepts(word),
            Automaton::NFA(nfa) => nfa.accepts(word),
        }
    }

    pub fn view(&self) {
        match self {
            Automaton::DFA(dfa) => dfa.view(),
            Automaton::NFA(nfa) => nfa.view(),
        }
    }
}
