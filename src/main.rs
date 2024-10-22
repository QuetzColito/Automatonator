use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;


trait Automaton {
    fn accepts(&self, word:&str) -> bool;
}

struct DFA {
    states: Vec<HashMap<char, usize>>,
    final_states: HashSet<usize>,
    start_state: usize
}

impl Automaton for DFA {
    fn accepts(&self, word:&str) -> bool {
        let mut current = self.start_state;
        let mut encountered_missing_edge = false;
        word.chars().for_each(|symbol:char| {
            if let Some(next) = self.states[current].get(&symbol) {
                current = *next;
            } else {
                encountered_missing_edge = true;
            }
        });
        self.final_states.contains(&current) && !encountered_missing_edge
    }
}

fn parse_text_automaton(file:String) -> Box<dyn Automaton> {
    let mut lines = file.lines();
    loop {
        let line = lines.next().expect("no type line found");
        let mut values = line.split_whitespace();
        if let Some(value) = values.next() {
            if value == "t" {
                return match values.next().expect("no actual type given in type line") {
                    "DFA" => {
                        let vertices = values.next().expect("missing vertices count")
                            .parse::<usize>().expect("vertices count not a number");
                        Box::new(parse_text_dfa(file, vertices))
                    },
                    _ => unimplemented!()
                }
            }
        }
    }
}

fn parse_text_dfa(file:String, vertices:usize) -> DFA {
        let mut automat = DFA {
            states: vec![HashMap::new(); vertices],
            final_states: HashSet::new(),
            start_state: 0,
        };
        file.lines().for_each(|line: &str| {
            println!("{line}");
            let mut values = line.split_whitespace();
            if let Some(value) = values.next() {
                match value {
                    "c" => (), // Comment, do nothing
                    "t" => (), // type is already covered, do nothing
                    "s" => {
                        automat.start_state = values.next().unwrap().parse::<usize>()
                            .expect("start state is not a number") - 1;
                    },
                    "f" => {
                        automat.final_states.insert(values.next().unwrap().parse::<usize>()
                            .expect("final state is not a number"));
                    },
                    _ => {
                        if let Ok(source) = value.parse::<usize>() {
                            let target = values.next().unwrap().parse::<usize>()
                                .expect("start state is not a number");
                            let label = values.next().unwrap().parse::<char>().unwrap_or('e');
                            automat.states[source - 1].insert(label, target - 1);
                        } else {
                            println!("ignored pattern {line}");
                        };
                    }
                }
            }
        });
        automat
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("No Automaton given");
    } else {
        println!("Reading Automaton from {}", args[1]);

        let file = fs::read_to_string(args[1].clone()).expect("file doesn't exist");
        let automat = parse_text_automaton(file);


        println!("{}", automat.accepts("aaa"));
        println!("{}", automat.accepts("aaaaaa"));
        println!("{}", automat.accepts("aaaaaaaaaa"));
    }
}
