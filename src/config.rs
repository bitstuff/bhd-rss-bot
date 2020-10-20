use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;
use serde::{Deserialize, Deserializer};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dropdir: String,
    pub frequency: u64,
    #[serde(rename(deserialize = "monitor"))]
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub url: String,
    #[serde(rename(deserialize = "match"))]
    pub matches: Vec<Match>,
}

fn regex_from_str<'de, D>(deserializer: D) -> Result<Regex, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let re = Regex::new(s.as_str())
        .expect(&format!("could not compile '{}' as a Regex", s));
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

pub fn new () -> Result<Config, Box<dyn Error>> {
    let path = format!("{}/.config/bhd-rss-bot/config.toml", env::var("HOME")?);
    let mut config_toml = String::new();
    let mut file = File::open(&path)?;
    file.read_to_string(&mut config_toml)?;
    let config: Config = toml::from_str(&config_toml)?;
    //println!("{:?}", config);
    return Ok(config);
}
