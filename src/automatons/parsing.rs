use std::fs;

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
    file.lines()
        .filter_map(|line: &str| {
            let mut values = line.split_whitespace();
            if let Some(value) = values.next() {
                match value {
                    "c" | "t" => None,
                    "s" => Some(AutomatonData::Start(
                        values
                            .next()
                            .expect("missing start state identifier")
                            .to_string(),
                    )),
                    "f" => Some(AutomatonData::Final(
                        values
                            .next()
                            .expect("missing final state identifier")
                            .to_string(),
                    )),
                    _ => {
                        if let Some(target) = values.next() {
                            Some(AutomatonData::Edge(
                                value.to_string(),
                                target.to_string(),
                                values.next().unwrap_or("e").to_string(),
                            ))
                        } else {
                            println!("ignored pattern {line}");
                            None
                        }
                    }
                }
            } else {
                println!("empty line");
                None
            }
        })
        .collect()
}

pub fn parse_xml(file: String) -> Vec<AutomatonData> {
    let data =
        roxmltree::Document::parse(&file).expect("XML Parsing Error (roxmltree threw an Error)");
    let automaton_data = data
        .descendants()
        .filter(|node| node.has_attribute("edge") || node.has_attribute("vertex"))
        .filter_map(|node| {
            if node.has_attribute("vertex") {
                if node
                    .parent()
                    .expect("root should have been filtered out by now")
                    .tag_name()
                    .name()
                    != "root"
                {
                    Some(AutomatonData::Final(
                        node.parent()
                            .unwrap()
                            .attribute("id")
                            .expect("final vertex without id")
                            .to_string(),
                    ))
                } else {
                    Option::None
                }
            } else {
                assert!(node.has_attribute("edge"));
                if node.has_attribute("source") && node.has_attribute("target") {
                    Some(AutomatonData::Edge(
                        node.attribute("source").unwrap().to_string(),
                        node.attribute("target").unwrap().to_string(),
                        node.attribute("value").unwrap_or("e").to_string(),
                    ))
                } else {
                    if node.has_attribute("target") || node.has_attribute("source") {
                        Some(AutomatonData::Start(
                            if let Some(id) = node.attribute("target") {
                                id.to_string()
                            } else {
                                node.attribute("source")
                                    .expect("source attribute existence checked earlier")
                                    .to_string()
                            },
                        ))
                    } else {
                        println!("Ignoring free floating edge");
                        None
                    }
                }
            }
        });
    automaton_data.collect()
}
