use chrono::{DateTime, FixedOffset};
use hashbrown::HashMap;
use serde;
use serde::{de, Deserialize, Deserializer};
use serde_xml_rs::{from_str};

fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_str(&s, "%a, %d %B %Y %H:%M:%S %z").map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
struct Item {
    title: String,
    #[serde(skip)]
    name: String,
    #[serde(skip)]
    category: String,
    #[serde(skip)]
    resolution: String,
    #[serde(skip)]
    size: String,
    link: String,
    comments: String,
    guid: String,
    #[serde(deserialize_with="naive_date_time_from_str", rename(deserialize = "pubDate"))]
    pubdate: DateTime<FixedOffset>,
}

#[derive(Debug, Deserialize)]
struct Channel {
    title: String,
    description: String,
    link: String,
    #[serde(deserialize_with="naive_date_time_from_str", rename(deserialize = "lastBuildDate"))]
    built: DateTime<FixedOffset>,
    #[serde(deserialize_with="naive_date_time_from_str", rename(deserialize = "pubDate"))]
    pubdate: DateTime<FixedOffset>,
    #[serde(rename(deserialize = "item"))]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct RSS {
    channel: Channel
}

fn main() {
    let resp = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<rss version=\"2.0\">
    <channel>
        <title>Beyond RSS</title>
        <description>RSS feed for the latest uploaded torrents to BeyondHD.</description>
        <link>https://beyond-hd.me/</link>
        <lastBuildDate>Sun, 18 Oct 2020 18:37:12 +0000</lastBuildDate>
        <pubDate>Sun, 18 Oct 2020 18:37:12 +0000</pubDate>
                            <item>
            <title>Foo Bar Baz 2020 720p BluRay DD5.1 x264-iFT / Movies / 720p / 5.41 GiB </title>
            <link>https:///download/foo-bar-baz-2020-720p-bluray-dd51-x264-ift.2</link>
            <comments>https:///foo-bar-baz-2020-720p-bluray-dd51-x264-ift.2</comments>
            <guid>https:///foo-bar-baz-2020-720p-bluray-dd51-x264-ift.2</guid>
            <pubDate>Sun, 18 Oct 2020 18:07:36 +0000</pubDate>
        </item>
                    <item>
            <title>Foo Bar Baz 2020 1080p BluRay DDP 5.1 x264-iFT / Movies / 1080p / 10.06 GiB </title>
            <link>https:///download/foo-bar-baz-2020-1080p-bluray-ddp-51-x264-ift.1</link>
            <comments>https:///foo-bar-baz-2020-1080p-bluray-ddp-51-x264-ift.1</comments>
            <guid>https:///foo-bar-baz-2020-1080p-bluray-ddp-51-x264-ift.1</guid>
            <pubDate>Sun, 18 Oct 2020 17:00:37 +0000</pubDate>
        </item>
  </channel>
</rss>
";
    let rss: RSS = from_str(&resp).unwrap();
    for item in rss.channel.items {
        println!("{}:  {}", item.pubdate, item.title);
    }
}