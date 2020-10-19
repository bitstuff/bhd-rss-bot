use chrono::{DateTime, FixedOffset};
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
    pub size: String,
    pub link: String,
    pub comments: String,
    pub guid: String,
    pub pubdate: DateTime<FixedOffset>,
}

impl std::convert::From<RawItem> for Item {
    fn from(raw: RawItem) -> Self {
        // split raw title into component pieces
        let pieces: Vec<String> = raw.title
            .split("/")
            .map(|x| x.to_string())
            .collect();
        // and build the resulting processed Item
        let item = Item {
            title:      raw.title,
            name:       pieces[0].clone(),
            category:   pieces[1].clone(),
            resolution: pieces[2].clone(),
            size:       pieces[3].clone(),
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

pub fn new (xml: &str) -> RSS {
    let rss: RSS = from_str(xml).unwrap();
    return rss;
}