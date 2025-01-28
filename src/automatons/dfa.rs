use super::automaton::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct DFA {
    states: HashMap<VertexId, HashMap<char, VertexId>>,
    alphabet: Vec<char>,
    final_states: HashSet<VertexId>,
    start_state: VertexId,
}

impl DFA {
    pub fn accepts(&self, word: &str) -> bool {
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

    pub fn view(&self) {
        println!("Type: DFA");
        println!(
            "Final States: {}",
            self.final_states
                .iter()
                .map(|id| format_id(id).to_string())
                .reduce(|acc, id| format!("{acc}, {id}"))
                .expect("Automaton should have at least 1 final state")
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
    pub fn new(data: Vec<AutomatonData>) -> DFA {
        let mut states = HashMap::new();
        let mut alphabet = HashSet::new();
        let mut final_states = HashSet::new();
        let mut start_state = "".to_string();
        data.into_iter().for_each(|d| match d {
            AutomatonData::Edge(source, target, label) => {
                let label = label.parse::<char>().unwrap_or('e');
                alphabet.insert(label);
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
        assert!(states.len() > 0, "No states given");
        assert!(final_states.len() > 0, "No final states given");
        assert_ne!(start_state, "", "No start state given");
        DFA {
            states,
            alphabet: alphabet.into_iter().collect(),
            final_states,
            start_state,
        }
    }

    pub fn alphabet(&self) -> &Vec<char> {
        &self.alphabet
    }
}
