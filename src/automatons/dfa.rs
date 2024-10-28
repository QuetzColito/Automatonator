use crate::automatons::shared::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct DFA {
    states: HashMap<VertexId, HashMap<char, VertexId>>,
    final_states: HashSet<VertexId>,
    start_state: VertexId,
}

impl Automaton for DFA {
    fn accepts(&self, word: &str) -> bool {
        let mut current = &self.start_state;
        let mut encountered_missing_edge = false;
        word.chars().for_each(|symbol: char| {
            if let Some(next) = self.states.get(current).unwrap().get(&symbol) {
                current = next;
            } else {
                encountered_missing_edge = true;
            }
        });
        self.final_states.contains(current) && !encountered_missing_edge
    }

    fn view(&self) {
        println!("Type: DFA");
        println!(
            "Final States: {}",
            self.final_states
                .iter()
                .map(|id| format_id(id).to_string())
                .reduce(|acc, id| format!("{acc}, {id}"))
                .unwrap()
        );
        println!("Start State: {}", format_id(&self.start_state));
        self.states.iter().for_each(|(id, map)| {
            println!("State {}:", format_id(id));
            map.iter().for_each(|(label, target)| {
                println!(
                    "    {} -> {}",
                    format_id(&label.to_string()),
                    format_id(target)
                )
            })
        })
    }
}

impl DFA {
    pub fn new(data: impl Iterator<Item = AutomatonData>) -> DFA {
        let mut states = HashMap::new();
        let mut final_states = HashSet::new();
        let mut start_state = "".to_string();
        data.for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                let label = label.parse::<char>().unwrap_or('e');
                states
                    .entry(source)
                    .or_insert(HashMap::new())
                    .insert(label, target);
            }
            AutomatonData::Final(id) => {
                final_states.insert(id);
            }
            AutomatonData::Start(id) => {
                if start_state != "" {
                    println!("multiple start states in a dfa, overwriting")
                };
                start_state = id;
            }
        });
        assert_ne!(start_state, "", "No start state given");
        DFA {
            states,
            final_states,
            start_state,
        }
    }
}
