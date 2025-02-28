use crate::shared::parsing::*;
use crate::tests::test_compare;

use super::test_against;

#[test]
fn parse_text() {
    // parse_automaton("data/nfa/is-uneven-dfa.gr", &None);
}

#[test]
fn parse_xml() {
    parse_automaton("data/nfa/equivNFA.drawio", &None);
    parse_automaton("data/nfa/equivNFA1.drawio.xml", &None);
    parse_automaton("data/nfa/equivNFA2.drawio.xml", &None);
}

#[test]
fn test_simulation() {
    test_against(
        "data/nfa/equivNFA1.drawio.xml",
        &["abbbba", "baa", "bbcbcc"],
        &["bccb", "acba", "abc", ""],
    );
}

#[test]
fn test_comparison() {
    let a1 = "data/nfa/equivNFA.drawio";
    let a2 = "data/nfa/equivNFA1.drawio.xml";
    let a3 = "data/nfa/equivNFA2.drawio.xml";

    test_compare(a1, a2, true);
    test_compare(a2, a3, true);
}
