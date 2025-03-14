#[cfg(test)]
use crate::shared::{evaluation::generated_comparison, parsing::parse_automaton};

#[cfg(test)]
pub mod dfa_test;

#[cfg(test)]
pub mod nfa_test;

#[cfg(test)]
pub mod pda_test;

#[cfg(test)]
pub mod generation_test;

#[cfg(test)]
fn test_against(filepath: &str, accept: &[&str], reject: &[&str]) {
    let a = parse_automaton(filepath, None).expect("testdata missing");

    accept.iter().for_each(|word| assert!(a.accepts(word)));

    reject.iter().for_each(|word| assert!(!a.accepts(word)));
}

#[cfg(test)]
fn test_compare(filepath: &str, filepath2: &str, equivalent: bool) {
    let a1 = parse_automaton(filepath, None).expect("testdata missing");
    let a2 = parse_automaton(filepath2, None).expect("testdata missing");
    assert!(generated_comparison(&a1, &a2) == if equivalent { 1 } else { 0 },);
}

#[cfg(test)]
fn view_test(filepaths: &[&str]) {
    for filepath in filepaths {
        parse_automaton(filepath, None)
            .expect("testdata missing")
            .view();
    }
}
