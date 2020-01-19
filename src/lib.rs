// --------------------------------------------------------------------------
//  stacorust.lib
// --------------------------------------------------------------------------

use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub lang: String,
    pub db_url: String,
}

impl Config {
    pub fn new(filename: &String) -> Result<Self, Box<dyn Error>> {
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        let config: Config = toml::from_str(&buffer)?;
        Ok(config)
    }
}
