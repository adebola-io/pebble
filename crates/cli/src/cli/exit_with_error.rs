pub fn exit_with_error(error: &str) -> ! {
    eprintln!("{}", error);
    std::process::exit(1)
}
