use log::info;

use super::automaton::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Split;

type Symbol = char;
type StackChar = char;

pub struct PDA {
    states: HashMap<VertexId, HashMap<(Symbol, StackChar), Vec<(VertexId, Vec<StackChar>)>>>,
    alphabet: Vec<char>,
    final_states: HashSet<VertexId>,
    start_states: HashSet<VertexId>,
}

impl PDA {
    pub fn accepts(&self, word: &str) -> bool {
        let mut currents: Vec<(VertexId, Vec<char>)> = self
            .start_states
            .clone()
            .into_iter()
            .map(|state| (state, vec!['#']))
            .collect();
        word.chars().for_each(|symbol| {
            currents = currents
                .iter_mut()
                .flat_map(|(state, stack)| {
                    if stack.len() == 0 {
                        return Vec::new();
                    }

                    let stack_char = stack.pop().unwrap();

                    let nexts = self.next_states(&state, symbol, stack_char);
                    match nexts.len() {
                        0 => Vec::new(),
                        1 => {
                            let (state, mut to_stack) = nexts[0].clone();
                            stack.append(&mut to_stack);
                            vec![(state, stack.to_vec())]
                        }
                        _ => nexts
                            .iter()
                            .map(|(state, to_stack)| {
                                let mut stack = stack.clone();
                                let mut to_stack = to_stack.clone();
                                stack.append(&mut to_stack);
                                (state.clone(), stack)
                            })
                            .collect(),
                    }
                })
                .collect()
        });
        currents
            .iter()
            .any(|(state, stack)| stack.len() == 0 && self.final_states.contains(state))
    }

    fn next_states(
        &self,
        state: &VertexId,
        symbol: Symbol,
        stack_char: StackChar,
    ) -> Vec<(VertexId, Vec<StackChar>)> {
        if let Some(nexts) = self
            .states
            .get(state)
            .and_then(|nexts| nexts.get(&(symbol, stack_char)))
        {
            nexts.to_vec()
        } else {
            Vec::new()
        }
    }

    pub fn view(&self) {
        println!("Type: NFA");
        println!("Final States: {}", format_states(&self.final_states));
        println!("Start States: {}", format_states(&self.start_states));
        self.states.iter().for_each(|(id, map)| {
            println!("State {}:", id);
            map.iter().for_each(|(label, target)| {
                println!(
                    "    {}, {} -> {}",
                    &label.0.to_string(),
                    &format_states_pda(&target),
                    &label.1.to_string(),
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
                info!("{label}");
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
            final_states,
            start_states,
        }
    }

    pub fn alphabet(&self) -> &Vec<char> {
        &self.alphabet
    }
}

fn parse_next(values: &mut Split<'_, &str>, error: &str) -> char {
    values.next().expect(error).trim().parse().unwrap_or(' ')
}

fn format_states_pda(states: &Vec<(VertexId, Vec<char>)>) -> String {
    states
        .iter()
        .map(|(id, to_stack)| id.to_string() + &to_stack.iter().collect::<String>())
        .reduce(|acc, id| format!("{acc}, {id}"))
        .unwrap()
}
