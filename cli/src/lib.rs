use std::collections::HashMap;
use wca::{Command, Routine};

mod commands;
use commands::benchmarks_run::*;
use commands::results_get::*;

/// A command that runs a command
pub fn grammar_form() -> Vec<Command> {
    vec![run_benchmarks_command(), get_results()]
}

/// A command that creates a command
pub fn executor_form() -> HashMap<String, Routine> {
    HashMap::from([
        ("benchmarks.run".to_owned(), Routine::new(benchmarks)),
        ("results.get".to_owned(), Routine::new(table)),
    ])
}
