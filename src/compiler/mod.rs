use super::cli::exit_with_error;
mod scanner;

pub fn run(file: String) {
    let error;
    let path = std::path::Path::new(&file);
    if !(path.exists() && path.is_file()) {
        error = format!(
            "Could not find the file \"{}\". Please check the input and try again.",
            file
        );
        exit_with_error(error.as_str());
    } else if path.extension().unwrap() != "peb" {
        error = format!("Cannot read {}. File is not a valid Pebble file.", file);
        exit_with_error(error.as_str());
    }
    println!("Compiling {}...", file);
    let content = std::fs::read_to_string(file).unwrap_or_else(|err| {
        let err = err.to_string();
        exit_with_error(err.as_str());
    });
    let tokens = scanner::scan(content).unwrap_or_else(|err| {
        println!(
            "Error encountered while scanning file on {}:{}",
            err.line, err.column
        );
        exit_with_error(err.message.as_str());
    });
    println!("{:?}", tokens);
}
