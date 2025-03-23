use log::info;

use crate::shared::automaton::*;
use crate::shared::utils::format_states;
use crate::shared::utils::logcheck_e;
use crate::shared::utils::parse_char;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type StackChar = char;
type Destinations = Vec<(VertexId, Stacks)>;
type Transitions = HashMap<Vec<StackChar>, Destinations>;

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
        // initialize state
        let mut seen: Vec<(VertexId, Stacks)> = self
            .start_states
            .clone()
            .into_iter()
            .map(|state| (state, Stacks::new(word, self.k)))
            .collect();
        let mut currents: VecDeque<_> = seen.clone().into();

        // check after every step if we accepted, or no new state was found
        while self.accepted(&currents) && !currents.is_empty() {
            // take steps for each current state
            for _ in 0..currents.len() {
                let current = currents.pop_front().unwrap();
                let stacks = current.1;
                let transitions = self.states.get(&current.0).unwrap();
                for next_key in transitions.keys() {
                    // check if transition can be applied to current stacks
                    if stacks.fits(next_key) {
                        // apply it to all destinations
                        for next in transitions.get(next_key).unwrap() {
                            let next = (next.0, stacks.clone().apply(&next.1));
                            // only apply state if it hasnt been seen yet
                            if !seen.contains(&next) {
                                seen.push(next.clone());
                                currents.push_back(next);
                            }
                        }
                    }
                }
            }
        }
        // return if while broke because it accepted or because no new currents
        self.accepted(&currents)
    }

    // helper to check for given automaton state if it is accepted
    fn accepted(&self, currents: &VecDeque<(VertexId, Stacks)>) -> bool {
        if self.final_states.is_empty() {
            currents.iter().any(|s| s.1.all_empty())
        } else {
            currents.iter().any(|s| self.final_states.contains(&s.0))
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
                    "\n    {} -> {}",
                    join_chars(label),
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

                for c in current_stacks.iter() {
                    if c.is_lowercase() {
                        alphabet.insert(*c);
                    }
                }

                logcheck_e(k == k_i, "Number of stacks not consistent.");
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .entry(current_stacks)
                    .or_insert(Vec::new())
                    .push((target, Stacks::from_data(next_stacks)));
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

#[derive(Clone, PartialEq, Eq)]
struct Stacks {
    data: Vec<String>,
}

impl Stacks {
    fn new(word: &str, k: usize) -> Self {
        assert!(k > 0);
        let mut data = Vec::new();
        data.push(word.to_string());
        for _ in 0..(k - 1) {
            data.push("#".to_string());
        }
        Stacks { data }
    }

    fn from_data(data: Vec<String>) -> Self {
        Stacks { data }
    }

    fn fits(&self, chars: &[char]) -> bool {
        self.data
            .iter()
            .zip(chars)
            .all(|(s, n)| *n == ' ' || s.ends_with(*n))
    }

    fn apply(mut self, other: &Stacks) -> Self {
        for i in 0..self.data.len() {
            self.data[i].push_str(&other.data[i]);
        }
        self
    }

    fn all_empty(&self) -> bool {
        self.data.iter().all(|s| s.is_empty())
    }
}

fn format_states_kpda(states: &[(VertexId, Stacks)]) -> String {
    states
        .iter()
        .map(|(id, to_stack)| id.to_string() + &to_stack.data.join(","))
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}

fn join_chars(chars: &[char]) -> String {
    chars
        .iter()
        .map(char::to_string)
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}
