use helper_functions::to_snake_case;
use relic::*;
use server_request::ClientConnector;
use server_request::get_server_string;

use chrono::offset::Utc;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::prelude::*;

use futures::Future;
use futures::Stream;

use hyper::Client;
use hyper::rt;
use hyper::Uri;
use hyper_tls::HttpsConnector;

use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;

use tokio_core::reactor::Core;

fn get_drop_data(client: &ClientConnector, core: &mut Core) -> String {
    get_server_string(client, core, "https://drops.warframestat.us/data/relics.json".parse().unwrap())
}

fn get_relic_metadata(client: &ClientConnector, core: &mut Core) -> String {
    get_server_string(client, core, "https://drops.warframestat.us/data/info.json".parse().unwrap())
}

fn update_relic_file(client: &ClientConnector, core: &mut Core) -> HashMap<String, Vec<HashMap<String, String>>> {
    let relic_items_map = get_relic_items_map(client, core);
    let relic_items_json = serde_json::to_value(&relic_items_map).unwrap();
    let current_time = Value::Number(serde_json::Number::from(Utc::now().timestamp_millis()));
    
    let mut top_level = HashMap::new();
    top_level.insert("timestamp", current_time);
    top_level.insert("data", relic_items_json);
    let top_level: Value = serde_json::to_value(top_level).unwrap();
    let top_level = format!("{}", top_level);

    let mut file = File::create("relic_drops.json").unwrap();
    file.write_all(top_level.as_bytes()).unwrap();

    relic_items_map
}

pub fn update_relic_items(client: &ClientConnector, core: &mut Core) -> HashMap<String, Vec<HashMap<String, String>>> {
    let file = File::open("relic_drops.json");
    let mut ret = None;
    match file {
        Ok(mut f) => {
            // Just read everything from the file if it is up-to-date
            let file_contents: Value = {
                let mut ret = String::new();
                f.read_to_string(&mut ret).unwrap();
                serde_json::from_str(&ret).unwrap()
            };

            let file_time = file_contents["timestamp"].as_i64().unwrap();

            let relic_server_meta = get_relic_metadata(client, core);
            let relic_server_meta: Value = serde_json::from_str(&relic_server_meta).unwrap();

            if file_time < relic_server_meta["timestamp"].as_i64().unwrap() {
                let relic_items_map = update_relic_file(client, core);
                ret = Some(relic_items_map);
            } else {
                let relic_items_map = serde_json::from_value(file_contents["data"].clone()).unwrap();
                ret = Some(relic_items_map);
            }
        },
        Err(_) => {
            // Assume the file doesn't exist, so let's create it
            let relic_items_map = update_relic_file(client, core);
            ret = Some(relic_items_map);
        }
    }

    ret.unwrap()
}

fn get_relic_items_map(client: &ClientConnector, core: &mut Core) -> HashMap<String, Vec<HashMap<String, String>>> {
    // Get the data from the server
    let relic_drops = get_drop_data(client, core);
    let relic_drops: Value = serde_json::from_str(&relic_drops).unwrap();
    let relic_drops = relic_drops["relics"].as_array().unwrap();

    // Map from Relic (Axi A4, Neo N9, ...) to [item_name+rarity+market_url]
    let mut ret: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();

    for v in relic_drops {
        // For each intact relic
        let state = v["state"].as_str().unwrap();
        if state != "Intact" {
            continue;
        }

        // Check what we get from it
        let tier = v["tier"].as_str().unwrap();
        let name = v["relicName"].as_str().unwrap();

        let tier = RelicTier::from_string(&tier).unwrap();
        let rewards = v["rewards"].as_array().unwrap();

        // Save each reward with the chance to get it
        for r in rewards {
            let item_name = r["itemName"].as_str().unwrap();
            let chance: String    = match r["chance"].clone() {
                Value::String(s) => s,
                Value::Number(n) => {
                    if n.is_u64() {
                        format!("{}", n.as_u64().unwrap())
                    } else {
                        // We assume this means we have a decimal, and therefore it must be 25.33
                        "25.33".to_owned()
                    }
                },
                _                => panic!("Got invalid value for chance."),
            };
            let rarity = rarity_from_intact_chance(&chance);
            let rarity = RelicRarity::from_string(&rarity).unwrap();

            let rewards_for_relic = ret.entry(format!("{} {}", tier.to_string(), name.clone())).or_insert(Vec::new());

            let mut curr_map = HashMap::new();
            curr_map.insert("rarity".to_owned(), rarity.to_string());
            curr_map.insert("market_url".to_owned(), remove_trailing_information(to_snake_case(&item_name)));
            curr_map.insert("name".to_owned(), item_name.to_owned());

            rewards_for_relic.push(curr_map);
        }
    }

    ret
}

fn remove_trailing_information(market_url: String) -> String {
    if (market_url.contains("chassis") || market_url.contains("neuroptics") || market_url.contains("systems")) && market_url.ends_with("blueprint") {
        // We have to remove the trailing "_blueprint" in these cases
        // "_blueprint".len() == 10
        market_url.chars().take(market_url.len() - 10).fold("".to_owned(), |acc, x| {
            format!("{}{}", acc, x)
        })
    } else {
        // Nothing special, just return what we got
        market_url
    }
}

fn rarity_from_intact_chance(chance: &str) -> String {
    match chance {
        "2"     => "Rare",
        "11"    => "Uncommon",
        "25.33" => "Common",
        s       => panic!("rarity_from_intact_chance got invalid chance: {}", s)
    }.to_owned()
}

