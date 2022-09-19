use std::error::Error;

use chrono::{DateTime, FixedOffset};
use regex::Regex;
use serde::{de, Deserialize, Deserializer};
use serde_xml_rs::{from_str};

fn date_time_from_str<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_str(&s, "%a, %d %B %Y %H:%M:%S %z").map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct RSS {
    pub channel: Channel
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub title: String,
    pub description: String,
    pub link: String,
    #[serde(deserialize_with="date_time_from_str", rename(deserialize = "lastBuildDate"))]
    pub built: DateTime<FixedOffset>,
    #[serde(deserialize_with="date_time_from_str", rename(deserialize = "pubDate"))]
    pub pubdate: DateTime<FixedOffset>,
    #[serde(rename(deserialize = "item"))]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(from = "RawItem")]
pub struct Item {
    pub title: String,
    pub name: String,
    pub category: String,
    pub resolution: String,
    pub size: u64,
    pub link: String,
    pub comments: String,
    pub guid: String,
    pub pubdate: DateTime<FixedOffset>,
}

impl std::convert::From<RawItem> for Item {
    fn from(raw: RawItem) -> Self {
        let ws_re  = Regex::new(r"^[[:blank:]]+(?P<t>.*)[[:blank:]]+$").unwrap();
        let size_re = Regex::new(r"^\D*(?P<d>[\d\.]+)\D*$").unwrap();
        // split raw title into component pieces
        let pieces: Vec<&str> = raw.title
            .split("/")
            .collect();
        // strip leading/trailing space from the pieces of the title
        let name       = ws_re.replace_all(pieces[0], "$t");
        let category   = ws_re.replace_all(pieces[1], "$t");
        let resolution = ws_re.replace_all(pieces[2], "$t");
        let ssize      = size_re.replace_all(pieces[3], "$d");
        let mut size   = ssize.parse::<f64>().unwrap();
        size *= 1024_f64 * 1024_f64 * 1024_f64;
        // and build the resulting processed Item
        let item = Item {
            title:      raw.title.clone(),
            name:       name.to_string(),
            category:   category.to_string(),
            resolution: resolution.to_string(),
            size:       size as u64,
            link:       raw.link,
            comments:   raw.comments,
            guid:       raw.guid,
            pubdate:    raw.pubdate,
        };
        return item;
    }
}

#[derive(Debug, Deserialize)]
struct RawItem {
    title: String,
    link: String,
    comments: String,
    guid: String,
    #[serde(deserialize_with="date_time_from_str", rename(deserialize = "pubDate"))]
    pubdate: DateTime<FixedOffset>,
}

pub fn new (xml: &str) -> Result<RSS, Box<dyn Error>> {
    let rss: RSS = from_str(xml)?;
    return Ok(rss);
}

#[test]
fn test_parser() {
    use std::io::prelude::*;
    use std::fs::File;
    let mut file = File::open("src/tests/rss-example.xml").expect("Unable to open test file");
    let mut xml = String::new();
    file.read_to_string(&mut xml).expect("Unable to read the file");
    let result = new(&xml);
    match result {
	Err(why) => panic!("{}", why),
	Ok(_rss) => assert!(true),
    }
}
