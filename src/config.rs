use std::env;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;
use serde::{Deserialize, Deserializer};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dropdir: String,
    #[serde(rename(deserialize = "monitor"))]
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub url: String,
    pub frequency: u32,
    #[serde(rename(deserialize = "match"))]
    pub matches: Vec<Match>,
}

fn regex_from_str<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let re = Regex::new(s.as_str()).unwrap();
    return Ok(re);
}

#[derive(Debug, Deserialize)]
pub struct Match {
    #[serde(deserialize_with="regex_from_str")]
    pub regex: Regex,
    pub category: String,
    pub resolution: String,
    pub max_size: u64,
}

pub fn new () -> Config {
    let path = format!("{}/.config/bhd-rss-bot/config.toml", env::var("HOME").unwrap());
    let mut config_toml = String::new();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_)  => {
            panic!("Could not find config file!")
        }
    };
    file.read_to_string(&mut config_toml)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));
    let config: Config = toml::from_str(&config_toml).unwrap();
    //println!("{:?}", config);
    return config;
}
