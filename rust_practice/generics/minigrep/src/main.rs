use minigrep::{run, Config};
use std::env;
use std::process;

/// search in file
/// # example
/// 
/// ```
/// minigrep [query] [filename]
/// ``` 
fn main() {
    let args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
