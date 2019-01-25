use market_prices::get_price_from_market;
use relic::*;
use server_request::ClientConnector;

use std::collections::HashMap;

use tokio_core::reactor::Core;

/// Function changes a string like "Hello There frieNd" to "hello_there_frie_nd". Assumes that the
/// given string slice is at least of length 1 and that the first character is a letter.
pub fn to_snake_case(s: &str) -> String {
    assert!(s.len() > 0);
    let first_char = &s.chars().next().unwrap().to_lowercase();
    let mut ret = Vec::new();
    ret.push(first_char.to_string());

    let mut last_was_space = false;

    for c in s.chars().skip(1) {
        if c.is_whitespace() {
            last_was_space = true;
            continue;
        }

        if last_was_space || c.is_uppercase() {
            ret.push("_".to_owned());
            last_was_space = false;
        }

        let c = match c {
            '&' => "and".to_owned(),
             c  => c.to_lowercase().to_string()
        };
        ret.push(c);
    }

    ret.iter().fold("".to_owned(), |acc, x| {
        format!("{}{}", acc, x)
    })
}

pub fn get_relic_expected_value(client: &ClientConnector, core: &mut Core, item_to_value: &mut HashMap<String, f64>, drops: &Vec<HashMap<String, String>>, relic_upgrade_level: RelicUpgrade) -> f64 {
    let mut relic_price = 0.0f64;

    for d in drops {
        let item_rarity = RelicRarity::from_string(&d.get("rarity").unwrap()).unwrap();
        let item_url = d.get("market_url").unwrap();
        
        let item_value = get_price_from_market(client, core, item_to_value, item_url);
        let chance = item_rarity.chance_for_upgrade(relic_upgrade_level) * 0.01;
        relic_price += chance * item_value;
    }

    relic_price
}

