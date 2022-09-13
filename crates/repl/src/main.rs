use analyser::Resolver;
// use interpreter::Interpreter;
use parser::{Parser, Provider, Scanner};
use std::io::{self, Write};

fn _collect_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|err| {
        eprintln!(
            "ERROR: Something went wrong while reading your input.\n {}",
            err
        );
        std::process::exit(1);
    });
    input
}

fn _run() {
    println!("Type a statement to evaluate.");
    loop {
        print!(">  ");
        io::stdout().flush().unwrap();
        let input = _collect_user_input();

        evaluate(input.trim_end());
    }
}

fn evaluate(input: &str) {
    let mut scanner = Scanner::new(input);
    scanner.run();

    let provider = Provider { scanner, index: 0 };
    let parser = Parser::new(provider);
    parser.parse();
    let resolver = Resolver::new(&parser);
    println!("");

    match resolver.resolve() {
        Err(errors) => {
            for err in errors {
                let message = format!("SyntaxError: {} at {}:{}", err.0, err.1[1][0], err.1[1][1]);
                eprintln!("{}", message);
            }
        }
        Ok(_) => {
            if resolver.diagnostics.borrow().len() > 0 {
                for err in resolver.diagnostics.borrow().iter() {
                    let error = format!(
                        "SemanticError: {} at {}:{}",
                        err.0, err.1[0][0], err.1[0][1]
                    );
                    println!("{}", error);
                }
            } else {
                println!("..");
            }
        }
    }
}

fn main() {
    _run();
}
