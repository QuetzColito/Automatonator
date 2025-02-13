use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::automaton::Automaton;

pub fn fixed_test(automaton: &Automaton, wordlist: &str) -> u64 {
    wordlist
        .lines()
        .map(|word| {
            if automaton.accepts(word) {
                println!("Accepted: '{}'", word);
                1
            } else {
                println!("Rejected: '{}'", word);
                0
            }
        })
        .sum()
}

pub fn fixed_comparison(
    automaton1: &Automaton,
    automaton2: &Automaton,
    wordlist: &str,
) -> (u64, u64) {
    wordlist
        .lines()
        .filter_map(|testcase| {
            let mut items = testcase.split_whitespace();
            if let Some(word) = items.next() {
                let points = items
                    .next()
                    .expect("no point value given")
                    .parse::<u64>()
                    .expect("points value not a number");
                if automaton1.accepts(word) == automaton2.accepts(word) {
                    Some((points, points))
                } else {
                    Some((0, points))
                }
            } else {
                println!("skipped an empty line");
                None
            }
        })
        .fold((0, 0), |(reached, max), (new_reached, new_max)| {
            (reached + new_reached, max + new_max)
        })
}

pub fn generated_comparison(automaton1: &Automaton, automaton2: &Automaton) -> u64 {
    let alphabet = automaton1.alphabet();
    let passed_generated = (0..10).all(|l| {
        (0..alphabet.len().pow(l)).all(|seed| {
            let word = make_word(seed as u64, l as usize, alphabet);
            dbg!(&word);
            automaton1.accepts(&word) == automaton2.accepts(&word)
        })
    });
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let passed_rng = (0..1000000).all(|_| {
        let len = rng.gen_range(0..25);
        let seed = rng.gen_range(0..alphabet.len().pow(len)) as u64;
        let word = make_word(seed, len as usize, alphabet);
        automaton1.accepts(&word) == automaton2.accepts(&word)
    });
    if passed_generated && passed_rng {
        1
    } else {
        0
    }
}

pub fn full_comparison(automaton1: &Automaton, automaton2: &Automaton, wordlist: &str) -> f64 {
    let (fixed_reached, fixed_max) = fixed_comparison(&automaton1, &automaton2, wordlist);
    let generated_reached = generated_comparison(&automaton1, &automaton2);
    (fixed_reached + generated_reached) as f64 / (fixed_max + 1) as f64
}

pub fn make_word(seed: u64, min_length: usize, alphabet: &Vec<char>) -> String {
    let mut seed = seed;
    let s = alphabet.len() as u64;
    let mut out = String::new();

    while seed >= s {
        let digit = seed % s;
        out.push(alphabet[digit as usize]);
        seed = seed / s;
    }
    out.push(alphabet[seed as usize]);

    while out.len() < min_length {
        out.push(alphabet[0]);
    }

    out
}
