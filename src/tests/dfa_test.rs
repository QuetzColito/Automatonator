#[cfg(test)]
use crate::shared::parsing::*;

#[test]
fn parse_text() {
    parse_automaton("data/dfa/is-uneven-dfa.gr".to_string(), &None);
}

#[test]
fn parse_xml() {
    parse_automaton("data/dfa/is-uneven-dfa.xml".to_string(), &None);
}
