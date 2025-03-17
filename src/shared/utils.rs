use log::{error, info, warn};

use super::automaton::VertexId;

pub fn format_states(states: &[VertexId]) -> String {
    states
        .iter()
        .map(|id| id.to_string())
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap_or("None".to_string())
}

pub fn parse_char(input: &str) -> char {
    let out = input.trim().parse().unwrap_or('e');
    if out == 'e' {
        info!("Parsing {} as epsilon", input);
        ' '
    } else {
        out
    }
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
