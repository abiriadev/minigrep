use std::{env, process};

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err_message| {
        eprintln!("{}", err_message);
        process::exit(1);
    });

    // println!("query: {}", config.query);
    // println!("filename: {}", config.filename);

    if let Err(err) = minigrep::run(config) {
        eprintln!("Can't read file data: {}", err);

        process::exit(1);
    }
}
