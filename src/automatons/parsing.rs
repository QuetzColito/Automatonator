use std::{collections::HashMap, fs};

use log::info;

use super::{automaton::*, dfa::DFA, nfa::NFA, pda::PDA};

pub fn parse_automaton(filepath: String, automaton_type: &Option<String>) -> Automaton {
    let file = fs::read_to_string(&filepath).expect("file doesn't exist");
    let automaton_type = determine_automaton_type(
        &automaton_type
            .clone() // .clone()
            .unwrap_or(path_to_automaton_type(&filepath)),
    );

    let automaton_data = if filepath.ends_with(".xml") || filepath.ends_with(".drawio") {
        parse_xml(file)
    } else {
        parse_text(file)
    };

    match automaton_type {
        AutomatonType::DFA => Automaton::DFA(DFA::new(automaton_data)),
        AutomatonType::NFA => Automaton::NFA(NFA::new(automaton_data)),
        AutomatonType::PDA => Automaton::PDA(PDA::new(automaton_data)),
    }
}

pub fn parse_text(file: String) -> Vec<AutomatonData> {
    let mut idgen = IdGenerator::new();
    file.lines()
        .filter_map(|line: &str| {
            let mut values = line.split_whitespace();
            if let Some(value) = values.next() {
                match value {
                    "c" | "t" => None,
                    "s" => Some(AutomatonData::Start(
                        idgen.get(values.next().expect("missing start state identifier")),
                    )),
                    "f" => Some(AutomatonData::Final(
                        idgen.get(values.next().expect("missing final state identifier")),
                    )),
                    _ => {
                        if let Some(target) = values.next() {
                            Some(AutomatonData::Edge(
                                idgen.get(value),
                                idgen.get(target),
                                values.next().unwrap_or("e").to_string(),
                            ))
                        } else {
                            info!("ignored pattern {line}");
                            None
                        }
                    }
                }
            } else {
                info!("empty line");
                None
            }
        })
        .collect()
}

pub fn parse_xml(file: String) -> Vec<AutomatonData> {
    let data =
        roxmltree::Document::parse(&file).expect("XML Parsing Error (roxmltree threw an Error)");
    let mut idgen = IdGenerator::new();
    // labels can be either as a value directly on the edge or as a separate vertex linking to the edge
    let labels: Vec<_> = data
        .descendants()
        .filter(|node| node.has_attribute("vertex") && node.has_attribute("style"))
        .filter(|node| node.attribute("style").unwrap().contains("edgeLabel"))
        .collect();
    let automaton_data = data
        .descendants()
        .filter(|node| node.has_attribute("edge") || node.has_attribute("vertex"))
        .flat_map(|node| {
            if node.has_attribute("vertex") {
                if node
                    .attribute("style")
                    .unwrap_or("NOSTYLE")
                    .contains("shape=doubleEllipse")
                {
                    vec![AutomatonData::Final(
                        idgen.get(
                            node.parent()
                                .unwrap()
                                .attribute("id")
                                .expect("final vertex without id"),
                        ),
                    )]
                } else {
                    Vec::new()
                }
            } else {
                assert!(node.has_attribute("edge"));
                if node.has_attribute("source") && node.has_attribute("target") {
                    sanitize_label(node.attribute("value").unwrap_or_else(|| {
                        labels
                            .iter()
                            .find(|label| label.attribute("parent") == node.attribute("id"))
                            .expect("edge without label")
                            .attribute("value")
                            .expect("label without value")
                    }))
                    .into_iter()
                    .map(|label| {
                        AutomatonData::Edge(
                            idgen.get(node.attribute("source").unwrap()),
                            idgen.get(node.attribute("target").unwrap()),
                            label,
                        )
                    })
                    .collect()
                } else {
                    if node.has_attribute("target") || node.has_attribute("source") {
                        vec![AutomatonData::Start(
                            if let Some(id) = node.attribute("target") {
                                idgen.get(id)
                            } else {
                                idgen.get(node.attribute("source").unwrap())
                            },
                        )]
                    } else {
                        println!("Ignoring free floating edge");
                        Vec::new()
                    }
                }
            }
        });
    automaton_data.collect()
}

fn sanitize_label(label: &str) -> Vec<String> {
    let label = label.replace("<br>", "");
    let label = label.replace("<div>", "");
    label
        .split("</div>")
        .map(|l| l.trim().to_owned())
        .filter(|l| l != "")
        .collect()
}

struct IdGenerator {
    id_map: HashMap<String, u32>,
    id: u32,
}

impl IdGenerator {
    fn get(&mut self, id_str: &str) -> u32 {
        if let Some(id) = self.id_map.get(id_str) {
            *id
        } else {
            self.id += 1;
            self.id_map.insert(id_str.to_owned(), self.id);
            self.id
        }
    }

    fn new() -> Self {
        IdGenerator {
            id_map: HashMap::new(),
            id: 0,
        }
    }
}
