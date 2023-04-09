use lako_interpreted::frontend::expr_ast::AstPrinter;
use lako_interpreted::frontend::parser::Parser;
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
            process::exit(5); // IO error
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

    // temp
    println!("{:?}", tokens);

    // let mut parser = Parser::new(tokens.to_vec());
    // let expr = match parser.parse() {
    //     Ok(res) => res,
    //     Err(e) => {
    //         eprintln!("Parsing error: {:?}", e);
    //         process::exit(127); // command not found or invalid command
    //     }
    // };
    // let mut printer = AstPrinter;
    // match printer.print(expr) {
    //     Ok(res) => println!("{}", res),
    //     Err(e) => eprintln!("Error {:?}", e),
    // };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: lako [file]");
            process::exit(64); // arguments error
        }
    }
}
