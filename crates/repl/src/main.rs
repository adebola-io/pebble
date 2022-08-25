use parser::{Parser, Provider, Scanner};
use std::io::{self, Write};

fn collect_user_input() -> String {
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

fn main() {
    println!("Type a statement to evaluate.");
    loop {
        print!(">  ");
        io::stdout().flush().unwrap();
        let input = collect_user_input();

        let mut scanner = Scanner::new(input.trim_end());
        scanner.run();

        let provider = Provider { scanner, index: 0 };
        let parser = Parser::new(provider);
        parser.parse();

        println!("");

        if parser.diagnostics.borrow().len() > 0 {
            for err in parser.diagnostics.borrow().iter() {
                let error = format!("SyntaxError: {} at {}:{}", err.0, err.1[1][0], err.1[1][1]);
                println!("{}", error);
            }
        } else {
            interpret(&parser)
        }
    }
}

fn interpret(parser: &Parser) {}
