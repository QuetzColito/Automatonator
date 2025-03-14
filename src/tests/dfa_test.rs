use crate::shared::parsing::*;
use crate::tests::test_compare;

use super::{test_against, view_test};

#[test]
fn parse_text() {
    parse_automaton("data/dfa/is-uneven-dfa.gr", None).unwrap();
    parse_automaton("data/dfa/dfa-empty-test.txt", None).unwrap();
}

#[test]
fn view() {
    view_test(&[
        "data/dfa/is-uneven-dfa.gr",
        "data/dfa/is-uneven-dfa.xml",
        "data/dfa/is-uneven-capped.drawio.xml",
        "data/dfa/carousel.drawio",
        "data/dfa/importantdfa.drawio.xml",
        "data/dfa/dfa-empty-test.txt",
    ]);
}

#[test]
fn parse_xml() {
    parse_automaton("data/dfa/is-uneven-dfa.xml", None).unwrap();
    parse_automaton("data/dfa/importantdfa.drawio.xml", None).unwrap();
    parse_automaton("data/dfa/is-uneven-capped.drawio.xml", None).unwrap();
    parse_automaton("data/dfa/carousel.drawio", None).unwrap();
}

#[test]
fn test_simulation() {
    test_against(
        "data/dfa/is-uneven-dfa.xml",
        &["a", "aaa", "aaaaa"],
        &["aa", "aaaa", "aaaaaa", ""],
    );
    test_against(
        "data/dfa/importantdfa.drawio.xml",
        &["uwu"],
        &["aa", "uwwu", "uwuw", ""],
    );
    test_against(
        "data/dfa/dfa-empty-test.txt",
        &["uwu", "", "uw", "uwuuuuu"],
        &["aa", "uwwu", "uwuw"],
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
    let a5 = "data/dfa/importantdfa.drawio.xml";
    let a6 = "data/dfa/dfa-empty-test.txt";

    test_compare(a1, a2, true);
    test_compare(a2, a3, false);
    test_compare(a3, a4, false);
    test_compare(a5, a6, false);
}
