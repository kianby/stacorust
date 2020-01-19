use std::io;
use std::io::prelude::*;
use std::fs::File;
use toml;
use serde::Deserialize;
//use serde::serde_derive::Deserialize;
//use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Config {
    lang: String,
    db_url: String
}

fn main() -> io::Result<()> {

    let mut f = File::open("configtest.toml")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let config: Config = toml::from_str(&buffer).unwrap();
    println!("lang = {}", config.lang);
    println!("db url = {}", config.db_url);
    Ok(())
}
