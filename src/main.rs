use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::thread::sleep;

use chrono::Utc;
use hashbrown::HashMap;
use regex::Regex;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut seen: HashMap<String, bool> = HashMap::new();
    let config = config::new()?;
    let sleep_time = Duration::new(config.frequency, 0);
    let file_re = Regex::new(r"(/[^/]+)$")?;
    loop {
        for monitor in &config.monitors {
            //println!("{:?}: polling {}", Utc::now(), monitor.name);
	    let rssxml = reqwest::get(&monitor.url).await?
                .text().await?;
            let rss = rss::new(&rssxml)
                .expect(&format!("Unable to parse RSS response for {}", monitor.name));
            for item in rss.channel.items {
                if seen.contains_key(&item.guid) {
                    continue;
                }
                for m in &monitor.matches {
                    if m.resolution == item.resolution
                          && m.category == item.category
                          && m.max_size > item.size
                          && m.regex.is_match(&item.name) {
                        let content = reqwest::get(&item.link).await?
                            .bytes().await?;
                        let mut filename = config.dropdir.clone();
                        //println!("guid: {}", item.guid);
                        let capture: Vec<regex::Captures> = file_re
                            .captures_iter(&item.guid)
                            .collect();
                        filename.push_str(&capture[0][0]);
                        filename.push_str(".torrent");
                        println!("{:?}: {}", Utc::now(), filename);
                        let mut file = File::create(filename)?;
                        file.write_all(&content)?;
                        break;
                    }
                }
                seen.insert(item.guid, true);
            }
        }
        sleep(sleep_time);
    }
}

mod config;
mod rss;
