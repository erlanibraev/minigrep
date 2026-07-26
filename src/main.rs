use std::{env, process};
use minigrep::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Проблема при разборе аргументов: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Проблема при выполнении: {}", e);
        process::exit(1);
    }
}


