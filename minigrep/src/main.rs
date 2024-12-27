use std::env;
use minigrep::Config;
use std::process;

fn main() {
    let config = Config::build(&mut env::args()).unwrap_or_else(|err|{
        eprintln!("Problem with passing the arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("{e}");
    }
}