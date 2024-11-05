use super::{automaton::*, dfa::DFA, nfa::NFA};

pub fn parse_automaton(file: String, automaton_type: AutomatonType, isXml: bool) -> Automaton {
    let automaton_data = if isXml {
        parse_xml(file)
    } else {
        parse_text(file)
    };

    // let mut lines = file.lines();
    // let mut arguments = lines
    //     .find(|value| value.trim().starts_with("t"))
    //     .expect("No Type Line given")
    //     .split_whitespace();
    //
    // arguments.next(); // drop the argument containing t
    //
    // return match arguments.next().expect("no actual type given in type line") {
    //     "DFA" => Automaton::DFA(DFA::new(parse_text(file))),
    //     _ => panic!("Type Unknown"),
    // };

    match automaton_type {
        AutomatonType::DFA => Automaton::DFA(DFA::new(automaton_data)),
        AutomatonType::NFA => Automaton::NFA(NFA::new(automaton_data)),
    }
}

pub fn parse_text(file: String) -> Vec<AutomatonData> {
    file.lines()
        .filter_map(|line: &str| {
            println!("{line}");
            let mut values = line.split_whitespace();
            if let Some(value) = values.next() {
                match value {
                    "c" | "t" => None, // type is already covered, do nothing
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
                    Some(AutomatonData::Start(
                        if let Some(id) = node.attribute("target") {
                            id.to_string()
                        } else {
                            node.attribute("source")
                                .expect("free floating edge")
                                .to_string()
                        },
                    ))
                }
            }
        });
    automaton_data.collect()
}
