extern crate minigrep;

use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problema parseando os argumentos: {}", err);
        process::exit(1)
    });

    println!("Argumento: {}\nArquivo: {}", config.query, config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
