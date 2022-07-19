use pebble::{cli::exit_with_error, compiler};
fn main() {
    let help = "
COMMANDS
compile <filename>    Run the Pebble compiler on an entry file.
";
    let mut args = std::env::args();
    args.next();
    let command = args.next().unwrap_or_else(|| {
        println!("{}", help);
        std::process::exit(0);
    });
    let command = command.as_str();
    match command {
        "compile" => {
            let file = args.next().unwrap_or_else(|| {
                exit_with_error("Expected file to compile. To compile a file, use cargo run compile <filename>.");
            });
            compiler::run(file);
        }
        _ => {
            eprintln!("Unknown command.");
            println!("{}", help);
            std::process::exit(1);
        }
    }
}
