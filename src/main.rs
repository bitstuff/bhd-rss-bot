use regex::Regex;
use reqwest;
use std::fs::File;
use std::io::prelude::*;

use hashbrown::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut seen: HashMap<String, bool> = HashMap::new();
    let config = config::new();

    let file_re = Regex::new(r"(/[^/]+)$")?;
    for monitor in &config.monitors {
        let rssxml = reqwest::blocking::get(&monitor.url)?
            .text()?;
        let rss = rss::new(&rssxml);
        for item in rss.channel.items {
            if seen.contains_key(&item.guid) {
                continue;
            }
            for m in &monitor.matches {
                if m.resolution == item.resolution
                      && m.category == item.category
                      && m.max_size > item.size
                      && m.regex.is_match(&item.name) {
                    let content = reqwest::blocking::get(&item.link)?
                        .bytes()?;
                    let mut filename = config.dropdir.clone();
                    //println!("guid: {}", item.guid);
                    let capture: Vec<regex::Captures> = file_re
                        .captures_iter(&item.guid)
                        .collect();
                    filename.push_str(&capture[0][0]);
                    filename.push_str(".torrent");
                    println!("fetching {} to {}", item.guid, filename);
                    let mut file = File::create(filename)?;
                    file.write_all(&content)?;
                    break;
                }
            }
            seen.insert(item.guid, true);
        }
    }
    
    Ok(())
}

mod config;
mod rss;