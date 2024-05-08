use std::env;
use std::process;
use todo::{run, Config, Todo};

fn main() {
    let todo = Todo::new().expect("Initialisation of DB Failed");
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem Parsing Arugments: {}", e);
        process::exit(1)
    });
    if let Err(e) = run(config, todo) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
