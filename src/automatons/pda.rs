use super::automaton::*;
use std::collections::HashMap;
use std::collections::HashSet;

struct State {}

type Symbol = char;
type StackChar = char;

pub struct PDA {
    states:
        HashMap<VertexId, HashMap<Symbol, HashMap<StackChar, HashSet<(VertexId, Vec<StackChar>)>>>>,
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
        word.chars().for_each(|symbol: char| {
            currents = currents
                .iter()
                .flat_map(|(state, stack)| {
                    self.states
                        .get(state)
                        .and_then(|nexts| nexts.get(&symbol))
                        .and_then(|nexts| nexts.get(&stack.last().unwrap()))
                        .and_then(|nexts| {
                            Some(
                                nexts
                                    .iter()
                                    .map(|(state, to_stack)| {
                                        let mut stack = stack.clone();
                                        let mut to_stack = to_stack.clone();
                                        stack.pop();
                                        stack.append(&mut to_stack);
                                        (state.clone(), stack)
                                    })
                                    .collect(),
                            )
                        })
                        .unwrap_or(HashSet::new())
                })
                .collect()
        });
        self.final_states
            .iter()
            .any(|f| currents.contains(&(f.clone(), vec![])))
    }

    pub fn view(&self) {
        println!("Type: NFA");
        println!("Final States: {}", format_states(&self.final_states));
        println!("Start States: {}", format_states(&self.start_states));
        self.states.iter().for_each(|(id, map)| {
            println!("State {}:", format_id(id));
            map.iter().for_each(|(label, target)| {
                target.iter().for_each(|(stackchar, target)| {
                    println!(
                        "    {}, {} -> {})",
                        format_id(&label.to_string()),
                        format_id(&format_states_pda(&target)),
                        format_id(&stackchar.to_string()),
                    )
                });
            })
        })
    }
    pub fn new(data: Vec<AutomatonData>) -> PDA {
        let mut states: HashMap<
            VertexId,
            HashMap<Symbol, HashMap<StackChar, HashSet<(VertexId, Vec<StackChar>)>>>,
        > = HashMap::new();
        let mut alphabet = HashSet::new();
        let mut final_states = HashSet::new();
        let mut start_states = HashSet::new();
        data.into_iter().for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                let mut values = label.split(",");
                let label = values
                    .next()
                    .expect("No Character given")
                    .trim()
                    .parse()
                    .unwrap_or('e');
                let current_stack = values
                    .next()
                    .expect("No Current Stackvalue given")
                    .trim()
                    .parse()
                    .unwrap_or('e');
                let next_stack = values.next().expect("No Next Stackvalue given").trim();
                alphabet.insert(label);
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .entry(label)
                    .or_insert(HashMap::new())
                    .entry(current_stack)
                    .or_insert(HashSet::new())
                    .insert((target, next_stack.chars().collect()));
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

fn format_states_pda(states: &HashSet<(VertexId, Vec<char>)>) -> String {
    "TODO".to_string()
}
