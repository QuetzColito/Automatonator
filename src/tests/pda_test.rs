use crate::shared::parsing::*;
use crate::tests::test_compare;

use super::{test_against, view_test};

#[test]
fn parse_text() {
    // parse_automaton("data/nfa/is-uneven-dfa.gr", &None);
}

#[test]
fn parse_xml() {
    parse_automaton("data/pda/pda.drawio.xml", None).unwrap();
    parse_automaton("data/pda/pdacompli.drawio.xml", None).unwrap();
}

#[test]
fn view() {
    view_test(&["data/pda/pda.drawio.xml", "data/pda/pdacompli.drawio.xml"]);
}

#[test]
fn test_simulation() {
    test_against(
        "data/pda/pda.drawio.xml",
        &["aaabbbb", "abb", "aabbb"],
        &["bccb", "aabba", "aaabb", ""],
    );
}

#[test]
fn test_comparison() {
    let a1 = "data/pda/pda.drawio.xml";
    let a2 = "data/pda/pdacompli.drawio.xml";

    test_compare(a1, a2, false);
}
