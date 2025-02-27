use std::collections::HashSet;

use log::{error, warn};

use super::automaton::VertexId;

pub fn format_states(states: &HashSet<VertexId>) -> String {
    states
        .iter()
        .map(|id| id.to_string())
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}

pub fn logcheck_w(value: bool, logtext: &str) {
    if value {
        warn!("{}", logtext)
    }
}

pub fn logcheck_e(value: bool, logtext: &str) {
    if value {
        error!("{}", logtext);
        panic!("{}", logtext)
    }
}
