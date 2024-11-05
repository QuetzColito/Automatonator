use super::automaton::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct NFA {
    states: HashMap<VertexId, HashMap<char, HashSet<VertexId>>>,
    final_states: HashSet<VertexId>,
    start_states: HashSet<VertexId>,
}

impl NFA {
    pub fn accepts(&self, word: &str) -> bool {
        let mut currents = self.start_states.clone();
        word.chars().for_each(|symbol: char| {
            currents = currents
                .iter()
                .flat_map(|current| {
                    if let Some(next) = self.states.get(current).unwrap().get(&symbol) {
                        next.clone()
                    } else {
                        HashSet::new()
                    }
                })
                .collect()
        });
        self.final_states.iter().any(|f| currents.contains(f))
    }

    pub fn view(&self) {
        println!("Type: DFA");
        println!("Final States: {}", format_states(&self.final_states));
        println!("Start States: {}", format_states(&self.start_states));
        self.states.iter().for_each(|(id, map)| {
            println!("State {}:", format_id(id));
            map.iter().for_each(|(label, target)| {
                println!(
                    "    {} -> {}",
                    format_id(&label.to_string()),
                    format_id(&format_states(&target))
                )
            })
        })
    }
    pub fn new(data: Vec<AutomatonData>) -> NFA {
        let mut states = HashMap::new();
        let mut final_states = HashSet::new();
        let mut start_states = HashSet::new();
        data.into_iter().for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                let label = label.parse::<char>().unwrap_or('e');
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .entry(label)
                    .or_insert(HashSet::new())
                    .insert(target);
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
            final_states,
            start_states,
        }
    }
}
