pub mod args;
pub mod automatons;
pub mod shared;
pub mod tests;

use log::*;
use shared::automaton::Automaton;
use std::fs;

use args::Args;
use clap::Parser;
use shared::evaluation::*;
use shared::parsing::*;
use std::time::Instant;

struct One {
    automaton: Automaton,
}

struct Two {
    a1: Automaton,
    a2: Automaton,
}

struct State<Automatons> {
    state: Automatons,
}

impl<A> State<A> {
    fn new(path: &str, atype: Option<String>) -> State<One> {
        info!("Reading Automaton from {}", path);

        let automaton = parse_automaton(path, atype).expect("Could not read first Automaton");

        info!("Successfully read Automaton:");
        automaton.view();

        State {
            state: One { automaton },
        }
    }
}

impl State<One> {
    fn add(
        self,
        path: Option<String>,
        atype1: Option<String>,
        atype2: Option<String>,
    ) -> Option<State<Two>> {
        if let Some(path) = path {
            info!("Reading Second Automaton from {}", path);

            if let Some(a2) = parse_automaton(&path, if atype2.is_some() { atype2 } else { atype1 })
            {
                info!("Successfully read Second Automaton:");
                a2.view();

                Some(State {
                    state: Two {
                        a1: self.state.automaton,
                        a2,
                    },
                })
            } else {
                warn!("Could not read Second Automaton, Skipping.");
                None
            }
        } else {
            None
        }
    }

    fn cases(&self, cases: Option<String>) -> &State<One> {
        // Test if Testcase File is given
        if let Some(cases) = cases.and_then(|path| fs::read_to_string(path).ok()) {
            info!("Evaluating Test Cases:");
            fixed_test(&self.state.automaton, &cases);
        }
        self
    }
}

impl State<Two> {
    fn evaluate(&self, eval_file: Option<String>) -> &State<Two> {
        info!("Comparing Automatons");
        // Evaluate if evaluation_file given
        if let Some(evaluation_file) = eval_file {
            let cases =
                fs::read_to_string(&evaluation_file).expect("evaluation file doesn't exist");

            println!(
                "Automaton reached {}% Points",
                full_comparison(&self.state.a1, &self.state.a2, &cases)
            );
        } else if generated_comparison(&self.state.a1, &self.state.a2) == 1 {
            info!("passed generated comparison")
        } else {
            warn!("did not pass generated comparison")
        }
        self
    }
}

fn main() {
    let now = Instant::now();
    let args = Args::parse();
    colog::init();

    // Read Single Automaton
    let state = State::<One>::new(&args.automaton, args.automaton_type.clone());
    // Test Test Cases if given
    state.cases(args.testcase_file);

    // Compare to Reference Automaton (if given)
    if let Some(state) = state.add(
        args.automaton2,
        args.automaton_type,
        args.ref_automaton_type,
    ) {
        state.evaluate(args.evaluation_file);
    }

    let elapsed = now.elapsed();
    info!("Took: {:.2?}", elapsed);
}
