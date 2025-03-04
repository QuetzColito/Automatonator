use log::info;
use log::warn;

use crate::shared::automaton::*;
use crate::shared::utils::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(clippy::upper_case_acronyms)]
pub struct NFA {
    states: HashMap<VertexId, HashMap<char, Vec<VertexId>>>,
    alphabet: Vec<char>,
    final_states: Vec<VertexId>,
    start_states: Vec<VertexId>,
}

impl NFA {
    pub fn accepts(&self, word: &str) -> bool {
        let mut currents: VecDeque<_> = self.start_states.iter().collect();
        for symbol in word.chars() {
            for _ in 0..currents.len() {
                let current = currents.pop_front().unwrap();
                if let Some(next) = self.states.get(current).and_then(|s| s.get(&symbol)) {
                    for s in next.iter() {
                        currents.push_back(s);
                    }
                }
            }
        }
        self.final_states.iter().any(|f| currents.contains(&f))
    }

    pub fn view(&self) {
        let mut out = String::new();
        out.push_str("Type: NFA");
        out.push_str(&format!(
            "\nFinal States: {}",
            format_states(&self.final_states)
        ));
        out.push_str(&format!(
            "\nStart States: {}",
            format_states(&self.start_states)
        ));
        let mut states: Vec<_> = self.states.iter().collect();
        states.sort_by_key(|&(key, _)| key);
        states.iter().for_each(|(id, map)| {
            out.push_str(&format!("\nState {}:", id));
            map.iter().for_each(|(label, target)| {
                out.push_str(&format!(
                    "\n    {} -> {}",
                    &label.to_string(),
                    &format_states(target)
                ))
            })
        });
        info!("{}", out);
    }
    pub fn new(data: Vec<AutomatonData>) -> NFA {
        let mut states = HashMap::new();
        let mut alphabet = HashSet::new();
        let mut final_states = HashSet::new();
        let mut start_states = HashSet::new();
        data.into_iter().for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                let label = label.parse::<char>().unwrap_or_else(|_| {
                    warn!("Parsing '{}' as epsilon, but epsilon transitions are not allowed in nfa, so will be ignored", label);
                    ' '
                });
                alphabet.insert(label);
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .entry(label)
                    .or_insert(Vec::new())
                    .push(target);
            }
            AutomatonData::Final(id) => {
                final_states.insert(id);
            }
            AutomatonData::Start(id) => {
                start_states.insert(id);
            }
        });
        assert!(!start_states.is_empty(), "No start state given");
        NFA {
            states,
            alphabet: alphabet.into_iter().collect(),
            final_states: final_states.into_iter().collect(),
            start_states: start_states.into_iter().collect(),
        }
    }

    pub fn alphabet(&self) -> &Vec<char> {
        &self.alphabet
    }
}
