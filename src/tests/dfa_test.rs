use crate::shared::parsing::*;
use crate::tests::test_compare;

use super::test_against;

#[test]
fn parse_text() {
    parse_automaton("data/dfa/is-uneven-dfa.gr", &None);
}

#[test]
fn parse_xml() {
    parse_automaton("data/dfa/is-uneven-dfa.xml", &None);
    parse_automaton("data/dfa/is-uneven-capped.drawio.xml", &None);
    parse_automaton("data/dfa/carousel.drawio", &None);
}

#[test]
fn test_simulation() {
    test_against(
        "data/dfa/is-uneven-dfa.xml",
        &["a", "aaa", "aaaaa"],
        &["aa", "aaaa", "aaaaaa", ""],
    );
    test_against(
        "data/dfa/carousel.drawio",
        &["caa", "cba", "cbdcaa"],
        &["cc", "", "caba", "cea"],
    );
}

#[test]
fn test_comparison() {
    let a1 = "data/dfa/is-uneven-dfa.gr";
    let a2 = "data/dfa/is-uneven-dfa.xml";
    let a3 = "data/dfa/is-uneven-capped.drawio.xml";
    let a4 = "data/dfa/carousel.drawio";

    test_compare(a1, a2, true);
    test_compare(a2, a3, false);
    test_compare(a3, a4, false);
}
