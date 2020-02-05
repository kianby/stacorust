// --------------------------------------------------------------------------
//  stacorust.main
// --------------------------------------------------------------------------

use std::env;

#[macro_use]
extern crate rust_embed;

mod config;
use config::Config;

mod template;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <config.toml>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("Read configuration = {}", &filename);

    let config = Config::new(filename).unwrap_or_else(|err| {
        println!("Configuration error: {}", err);
        std::process::exit(1)
    });

    println!("lang = {}", config.lang);
    println!("db url = {}", config.db_url);

    let something = template::get_template_new_comment(config.lang, "http://blog".to_string(), "Mon commentaire".to_string());
    match something {
        None => println!("Not found"),
        Some(template) => println!("{:?}", template),
    }

    // if let Err(e) = run(config) {
    //     println!("Application error: {}", e);
    //     std::process::exit(1);
    // }
}
