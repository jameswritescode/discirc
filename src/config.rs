use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub application_id: u64,
    pub token: String,
}

impl Config {
    pub fn load() -> Config {
        let mut file = File::open("discirc.toml").expect("Config Error");
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Config Read Error");

        let config: Config = toml::from_str(&contents).unwrap();

        return config;
    }
}
