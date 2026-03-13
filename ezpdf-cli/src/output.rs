pub fn print_success(msg: &str, quiet: bool) {
    if !quiet {
        println!("{msg}");
    }
}
