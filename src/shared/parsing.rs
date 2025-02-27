use std::{collections::HashMap, fs, vec};

use log::info;
use roxmltree::Node;

use crate::automatons::{dfa::DFA, nfa::NFA, pda::PDA};

use super::automaton::*;

pub fn parse_automaton(filepath: String, automaton_type: &Option<String>) -> Automaton {
    let file = fs::read_to_string(&filepath).expect("file doesn't exist");
    let automaton_type = determine_automaton_type(
        &automaton_type
            .clone()
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

fn parse_text(file: String) -> Vec<AutomatonData> {
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

fn parse_xml(file: String) -> Vec<AutomatonData> {
    let data =
        roxmltree::Document::parse(&file).expect("XML Parsing Error (roxmltree threw an Error)");
    let mut idgen = IdGenerator::new();
    // labels can be either as a value directly on the edge or as a separate vertex linking to the edge
    // we extract the labels that are vertices early to be able to loop over them when needed
    let labels: Vec<_> = data
        .descendants()
        .filter(|node| node.has_attribute("vertex") && has_style(node, "edgeLabel"))
        .collect();
    // Look through all nodes
    data.descendants()
        .filter(|node| node.has_attribute("edge") || node.has_attribute("vertex"))
        .flat_map(|node| {
            // Find final states
            if node.has_attribute("vertex") {
                if has_style(&node, "shape=doubleEllipse") {
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
                // Parse edges
                assert!(node.has_attribute("edge"));
                if node.has_attribute("source") && node.has_attribute("target") {
                    let id = node.attribute("id").expect("label without id");
                    let mut label = node
                        .attribute("value") // check if edge has label as value
                        .unwrap_or_else(|| find_related_label(id, &labels)); // else try to find a label
                    if label.is_empty() {
                        // make sure label didnt have an empty value
                        label = find_related_label(id, &labels)
                    }
                    sanitize_label(label) // might split label up into multiple lines
                        .into_iter()
                        .flat_map(|label| {
                            // prepare both to make last part more readable
                            let forward = AutomatonData::Edge(
                                idgen.get(node.attribute("source").unwrap()),
                                idgen.get(node.attribute("target").unwrap()),
                                label.clone(),
                            );
                            let backward = AutomatonData::Edge(
                                idgen.get(node.attribute("target").unwrap()),
                                idgen.get(node.attribute("source").unwrap()),
                                label,
                            );
                            // check arrow direction
                            // Default for startarrow is none, while default for end arrow is defaultArrow
                            let has_end_arrow = !has_style(&node, "endarrow=none");
                            let has_start_arrow = has_style(&node, "startarrow=")
                                && !has_style(&node, "startarrow=none");

                            // 3 Different Scenarios
                            if has_start_arrow == has_end_arrow {
                                vec![forward, backward]
                            } else if has_start_arrow {
                                vec![backward]
                            } else {
                                assert!(has_end_arrow, "Logic Error in Arrow detection");
                                vec![forward]
                            }
                        })
                        .collect()
                } else if node.has_attribute("target") || node.has_attribute("source") {
                    // Edge only connected to 1 Vertex, this is a start identifier
                    vec![AutomatonData::Start(
                        // must be either source or target
                        idgen.get(
                            node.attribute("target")
                                .unwrap_or_else(|| node.attribute("source").unwrap()),
                        ),
                    )]
                } else {
                    info!("Ignoring free floating edge");
                    Vec::new()
                }
            }
        })
        .collect()
}

// Looks for a label which parent is the given id of an Edge
// Panics if a label doesnt have a parent or it cant find a label
fn find_related_label<'a>(id: &'a str, labels: &'a Vec<Node<'_, '_>>) -> &'a str {
    info!("{}", id);
    labels
        .iter()
        .find(|label| label.attribute("parent").expect("label without parent") == id)
        .expect("edge without label")
        .attribute("value")
        .expect("label without value")
}

// Helper function to check if the style of a Node contains a str
fn has_style(node: &Node<'_, '_>, style: &str) -> bool {
    node.attribute("style")
        .unwrap_or("NOSTYLE")
        .to_lowercase()
        .contains(&style.to_lowercase())
}

// Split the weird multiline label syntax of drawio and remove the tags
fn sanitize_label(label: &str) -> Vec<String> {
    let label = label.replace("<br>", "");
    label
        .split("div")
        .map(|l| {
            l.replace("<", "")
                .replace(">", "")
                .replace("/", "")
                .trim()
                .to_owned()
        })
        .filter(|l| !l.is_empty())
        .collect()
}

// Maps all ids to simpler numbers
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
            info!("{} gets mapped to {}", id_str, self.id);
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
