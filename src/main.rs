use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args = env::args();
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let config = Config::build(args, ignore_case).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
