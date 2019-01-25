extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod helper_functions;
mod market_prices;
mod relic;
mod server_request;
mod update_from_server;

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use helper_functions::get_relic_expected_value;
use market_prices::get_price_from_market;
use update_from_server::update_relic_items;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use std::fs::File;

use relic::RelicDrop;
use relic::RelicTier;
use relic::RelicUpgrade;

use serde_json::Value;

fn main() {
    // Core is the Tokio event loop used for making a non-blocking request
    let mut core = Core::new().unwrap();

    let https = HttpsConnector::new(4).unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let relic_to_drops = update_relic_items(&client, &mut core);
    let mut item_to_value = HashMap::new();

    // Insert the items that we know can't be sold with price 0 into the map.
    item_to_value.insert("forma_blueprint".to_owned(), 0.0);

    // Ask user for relics or quit upon receiving specific strings
    let stdin = io::stdin();

    println!("Enter relic:");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "quit" || line == ":q" {
            break;
        }

        // Parse the relic
        let line: Vec<&str> = line.trim().split(' ').collect();
        if line.len() != 2 {
            println!("Invalid Relic. Enter '[tier] [name]', where tier is one of:");
            println!("  * Lith\n  * Meso\n  * Neo\n  * Axi");
            continue;
        }

        let tier = RelicTier::from_string(&line[0]);
        let name = line[1];
        match tier {
            Err(msg)    => println!("{}", msg),
            Ok(tier)    => {
                let relic_name = format!("{} {}", tier.to_string(), name);
                if let Some(drops) = relic_to_drops.get(&relic_name) {
                    for upgrade in RelicUpgrade::all_upgrade_tiers() {
                        let price = get_relic_expected_value(&client, &mut core, &mut item_to_value, &drops, upgrade);
                        println!("This relic is worth {}p when {}.", price, upgrade.to_string());
                    }
                } else {
                    println!("Please check the name of the relic, we can't find it...");
                }
            },
        }

        println!("Enter relic:");
    }
}

