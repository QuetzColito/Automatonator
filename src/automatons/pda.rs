use crate::shared::automaton::*;
use crate::shared::utils::format_states;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::Split;

type Symbol = char;
type StackChar = char;
type Destinations = Vec<(VertexId, Vec<StackChar>)>;
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
        let mut currents: Vec<(VertexId, Vec<char>)> = self
            .start_states
            .clone()
            .into_iter()
            .map(|state| (state, vec!['#']))
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
                        stack.append(&mut next.1.clone());
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
                                    stack.append(&mut next.1.clone());
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
                        stack.append(&mut next.1.clone());
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
        currents
            .iter()
            .any(|(state, stack)| stack.is_empty() && self.final_states.contains(state))
    }

    pub fn view(&self) {
        println!("Type: PDA");
        println!("Final States: {}", format_states(&self.final_states));
        println!("Start States: {}", format_states(&self.start_states));
        let mut states: Vec<_> = self.states.iter().collect();
        states.sort_by_key(|&(key, _)| key);
        states.iter().for_each(|(id, map)| {
            println!("State {}:", id);
            map.iter().for_each(|(label, target)| {
                println!(
                    "    {} {} -> {}",
                    &label.0.to_string(),
                    &label.1.to_string(),
                    &format_states_pda(target),
                )
            });
        })
    }
    pub fn new(data: Vec<AutomatonData>) -> PDA {
        let mut states = HashMap::new();
        let mut alphabet = HashSet::new();
        let mut final_states = HashSet::new();
        let mut start_states = HashSet::new();
        data.into_iter().for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                // info!("{label}");
                let mut values = label.split(",");
                let label = parse_next(&mut values, "No Character given");
                let current_stack = parse_next(&mut values, "No Current Stackvalue given");
                let next_stack = values.next().expect("No Next Stackvalue given").trim();
                alphabet.insert(label);
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .entry((label, current_stack))
                    .or_insert(Vec::new())
                    .push((target, next_stack.chars().collect()));
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
    values.next().expect(error).trim().parse().unwrap_or(' ')
}

fn format_states_pda(states: &[(VertexId, Vec<char>)]) -> String {
    states
        .iter()
        .map(|(id, to_stack)| id.to_string() + &to_stack.iter().collect::<String>())
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}
