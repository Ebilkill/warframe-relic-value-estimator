use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;

use std::collections::HashMap;

use tokio_core::reactor::Core;

use server_request::ClientConnector;
use server_request::get_server_string;

/// Returns the median price of yesterday for the given item. Note that this is liable to change
/// later.
pub fn get_price_from_market(client: &ClientConnector, core: &mut Core, item_to_value: &mut HashMap<String, f64>, item_url: &str) -> f64 {
    if let Some(price) = item_to_value.get(item_url) {
        return *price;
    }

    let statistics_json = get_server_string(client, core, format!("https://api.warframe.market/v1/items/{}/statistics", item_url).parse().unwrap());
    let statistics_json: Value = serde_json::from_str(&statistics_json).unwrap();
    let statistics_json: &Value = &statistics_json["payload"]["statistics_closed"]["90days"];
    let statistics_json: Value = {
        let v = statistics_json.as_array().unwrap();
        v.iter().last().unwrap().clone()
    };

    let value = statistics_json["median"].as_f64().unwrap();
    item_to_value.insert(item_url.to_owned(), value);

    value
}

