use log::info;

use crate::shared::automaton::*;
use crate::shared::utils::format_states;
use crate::shared::utils::logcheck_e;
use crate::shared::utils::parse_char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::Split;

type Symbol = char;
type StackChar = char;
type Destinations = Vec<(VertexId, Vec<String>)>;
type Transitions = HashMap<(Symbol, Vec<StackChar>), Destinations>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Stack {
    data: Vec<String>,
}

impl Stack {
    fn new(k: usize) -> Self {
        return Stack {
            data: vec!["#".to_string(); k],
        };
    }

    fn push_str(&mut self, next: &[String]) {
        let mut i = 0;
        for s in self.data.iter_mut() {
            s.push_str(&next[i]);
            i += 1;
        }
    }

    fn pop(&mut self) -> Option<Vec<StackChar>> {
        self.data.iter_mut().map(|s| s.pop()).collect()
    }

    fn is_empty(&self) -> bool {
        self.data.iter().all(String::is_empty)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct KPDA {
    states: HashMap<VertexId, Transitions>,
    alphabet: Vec<char>,
    final_states: Vec<VertexId>,
    start_states: Vec<VertexId>,
    k: usize,
}

impl KPDA {
    pub fn accepts(&self, word: &str) -> bool {
        let mut currents: Vec<(VertexId, Stack)> = self
            .start_states
            .clone()
            .into_iter()
            .map(|state| (state, Stack::new(self.k)))
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

                    for next in epsilonstates.into_iter() {
                        if !new.contains(&next) {
                            new.push(next);
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
                    &label.1.join(','),
                    &format_states_kpda(target),
                ))
            });
        });
        info!("{}", out);
    }
    pub fn new(data: Vec<AutomatonData>) -> KPDA {
        let mut states = HashMap::new();
        let mut alphabet = HashSet::new();
        let mut final_states = HashSet::new();
        let mut start_states = HashSet::new();
        let mut k = 0;
        data.into_iter().for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                // info!("{label}");
                let mut values = label.split(",");
                let symbol = parse_char(values.next().expect("No Character given"));
                let mut current_stacks = Vec::new();
                let mut next_stacks = Vec::new();
                let mut k_i = 0;
                while let Some(stack_char) = values.next() {
                    k_i += 1;
                    current_stacks.push(parse_char(stack_char));
                    next_stacks.push(
                        values
                            .next()
                            .expect("Even Number of Values in PDA Label not supported")
                            .to_owned(),
                    );
                }
                if k == 0 {
                    k = k_i
                }

                logcheck_e(k == k_i, "Number of stacks not consistent.");
                alphabet.insert(symbol);
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .entry((symbol, current_stacks))
                    .or_insert(Vec::new())
                    .push((target, next_stacks));
            }
            AutomatonData::Final(id) => {
                final_states.insert(id);
            }
            AutomatonData::Start(id) => {
                start_states.insert(id);
            }
        });
        assert!(!start_states.is_empty(), "No start state given");
        KPDA {
            states,
            alphabet: alphabet.into_iter().collect(),
            final_states: final_states.into_iter().collect(),
            start_states: start_states.into_iter().collect(),
            k: 1,
        }
    }

    pub fn alphabet(&self) -> &Vec<char> {
        &self.alphabet
    }
}

fn format_states_kpda(states: &[(VertexId, Vec<String>)]) -> String {
    states
        .iter()
        .map(|(id, to_stack)| id.to_string() + &to_stack.join(","))
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}
