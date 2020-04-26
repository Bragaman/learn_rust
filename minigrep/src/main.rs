use std::{env, process};
use minigrep::Config;

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    let result = minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    println!("search result:\n{:#?}", result)

}
