use log::info;

use crate::shared::automaton::*;
use crate::shared::utils::format_states;
use crate::shared::utils::parse_char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::Split;

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
            for current in currents.into_iter() {
                let state = current.0;
                let mut stack = current.1;
                let stack_char = stack.pop();
                if let Some(nexts) = self
                    .states
                    .get(&state)
                    .and_then(|s| stack_char.and_then(|stack_char| s.get(&(symbol, stack_char))))
                {
                    for next in nexts.iter() {
                        let mut stack = stack.clone();
                        stack.push_str(&next.1);
                        if !new.contains(&(next.0, stack.clone())) {
                            new.push((next.0, stack));
                        }
                    }

                    let mut epsilonstates = VecDeque::new();
                    epsilonstates.push_back((state, stack.clone()));

                    let mut found_new = true;
                    while found_new {
                        found_new = false;
                        for _ in 0..epsilonstates.len() {
                            let (state, mut stack) = epsilonstates.pop_front().unwrap();
                            if let Some(nexts) = self.states.get(&state).and_then(|s| {
                                stack.pop().and_then(|stack_char| s.get(&(' ', stack_char)))
                            }) {
                                for next in nexts.iter() {
                                    let mut stack = stack.clone();
                                    stack.push_str(&next.1);
                                    if !epsilonstates.contains(&(next.0, stack.clone())) {
                                        found_new = true;
                                        epsilonstates.push_back((next.0, stack));
                                    }
                                }
                            }
                        }
                    }

                    for next in epsilonstates.iter() {
                        let mut stack = stack.clone();
                        stack.push_str(&mut next.1.clone());
                        if !new.contains(&(next.0, stack.clone())) {
                            new.push((next.0, stack));
                        }
                    }
                }
            }
            new.sort_unstable();
            new.dedup();
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
                let label = parse_char(values.get(0).expect("No Character given"));
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

fn parse_next(values: &mut Split<'_, &str>, error: &str) -> char {
    let label = values.next().expect(error).trim();
    label.parse().unwrap_or_else(|_| {
        info!("Parsing '{}' as epsilon", label);
        ' '
    })
}

fn format_states_pda(states: &[(VertexId, String)]) -> String {
    states
        .iter()
        .map(|(id, to_stack)| id.to_string() + to_stack)
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}
