use std::{env, process};
use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Проблема при разборе аргументов: {}", err);
        process::exit(1);
    });
    println!("Поиск {}", config.query);
    println!("В файле {}", config.filename);

    if let Err(e) = run(config) {
        println!("Проблема при выполнении: {}", e);
        process::exit(1);
    }
}


