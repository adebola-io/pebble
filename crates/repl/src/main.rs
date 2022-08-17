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
    println!("Type \".help\" for more information.");
    print!(">  ");
    io::stdout().flush().unwrap();
    let input = collect_user_input();
    println!("{}", input);
}
