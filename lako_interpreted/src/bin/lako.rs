use lako_interpreted::frontend::scanner::Scanner;
use std::{
    env, fs,
    io::{self, Write},
    process,
};

fn run_file(path: &str) {
    let input = fs::read_to_string(path);
    match input {
        Ok(bytes) => run(bytes),
        Err(e) => {
            eprintln!("Failed to read file: {:?}", e);
            process::exit(74);
        }
    }
}

fn run_repl() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin!");

        run(input);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: lako [file]");
            process::exit(64);
        }
    }
}
