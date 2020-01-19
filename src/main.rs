// --------------------------------------------------------------------------
//  stacorust.main
// --------------------------------------------------------------------------

extern crate stacorust;

use stacorust::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <config.toml>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("filename = {}", &filename);

    let config = Config::new(filename).unwrap_or_else(|err| {
        eprintln!("Problem reading configuration: {}", err);
        std::process::exit(1)
    });

    println!("lang = {}", config.lang);
    println!("db url = {}", config.db_url);

    // if let Err(e) = run(config) {
    //     println!("Application error: {}", e);
    //     std::process::exit(1);
    // }
}
