use log::info;

use crate::shared::automaton::*;
use crate::shared::utils::format_states;
use crate::shared::utils::parse_char;
use std::collections::HashMap;
use std::collections::HashSet;

type Symbol = char;
type StackChar = char;
type Destinations = Vec<(VertexId, String)>;
type Transitions = HashMap<(Symbol, StackChar), Destinations>;

#[allow(clippy::upper_case_acronyms)]
pub struct PDA {
    states: HashMap<VertexId, Transitions>,
    alphabet: Vec<char>,
    final_states: Vec<VertexId>,
    start_states: Vec<VertexId>,
}

impl PDA {
    pub fn accepts(&self, word: &str) -> bool {
        let mut currents: Vec<(VertexId, String)> = self
            .start_states
            .clone()
            .into_iter()
            .map(|state| (state, "#".to_string()))
            .collect();
        for symbol in word.chars() {
            let mut new = Vec::new();
            let mut seen_states = currents.clone();
            while let Some(current) = currents.pop() {
                let state = current.0;
                let mut stack = current.1;
                let stack_char = stack.pop();
                let read_char = |c| {
                    self.states
                        .get(&state)
                        .and_then(|s| stack_char.and_then(|stack_char| s.get(&(c, stack_char))))
                };
                // epsilon transitions
                if let Some(nexts) = read_char(' ') {
                    for next in nexts.clone().into_iter() {
                        let mut next = next;
                        next.1 = stack.clone() + &next.1;
                        if !seen_states.contains(&next) {
                            seen_states.push(next.clone());
                            currents.push(next);
                        }
                    }
                }
                // non-epsilon transitions
                if let Some(nexts) = read_char(symbol) {
                    for next in nexts.clone().into_iter() {
                        let mut next = next;
                        next.1 = stack.clone() + &next.1;
                        if !new.contains(&next) {
                            new.push(next);
                        }
                    }
                }
            }
            currents = new;
        }

        if self.final_states.is_empty() {
            currents.iter().any(|(_, stack)| stack.is_empty())
        } else {
            currents
                .iter()
                .any(|(state, _)| self.final_states.contains(state))
        }
    }

    pub fn view(&self) {
        let mut out = String::new();
        out.push_str("Type: PDA");
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
                    "\n    {} {} -> {}",
                    &label.0.to_string(),
                    &label.1.to_string(),
                    &format_states_pda(target),
                ))
            });
        });
        info!("{}", out);
    }
    pub fn new(data: Vec<AutomatonData>) -> PDA {
        let mut states = HashMap::new();
        let mut alphabet = HashSet::new();
        let mut final_states = HashSet::new();
        let mut start_states = HashSet::new();
        data.into_iter().for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                let values: Vec<_> = label.split(",").collect();
                let label = parse_char(values.first().expect("No Character given"));
                let current_stack = parse_char(values.get(1).expect("No Current Stackvalue given"));
                let next_stack = values.get(2).expect("No Next Stackvalue given");
                alphabet.insert(label);
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .entry((label, current_stack))
                    .or_insert(Vec::new())
                    .push((target, next_stack.to_string()));
            }
            AutomatonData::Final(id) => {
                final_states.insert(id);
            }
            AutomatonData::Start(id) => {
                start_states.insert(id);
            }
        });
        assert!(!start_states.is_empty(), "No start state given");
        PDA {
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

fn format_states_pda(states: &[(VertexId, String)]) -> String {
    states
        .iter()
        .map(|(id, to_stack)| id.to_string() + to_stack)
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}
