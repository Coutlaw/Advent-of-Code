use std::env;
use std::process;

use day_2::Config;

fn main() {
    // config Result unwrapping with closure to handle Err
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run the src/lib.rs to open the config and run the program
    if let Err(e) = day_2::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

/*
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
*/
