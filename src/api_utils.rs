use std::collections::HashMap;

pub fn json_list(list: Vec<String>) -> String {
    let mut json = String::from("[");
    for (i, item) in list.iter().enumerate() {
        json.push_str(&format!("\"{}\"", item));
        if i < list.len() - 1 {
            json.push_str(",");
        }
    }
    json.push_str("]");
    json
}

pub fn json_parse_key_values(json: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let json = json.trim_start_matches("{").trim_end_matches("}");
    for pair in json.split(",") {
        let mut key_value = pair.split(":");
        let key = key_value.next().unwrap().trim().trim_matches('"');
        let value = key_value.next().unwrap().trim().trim_matches('"');
        map.insert(key.to_string(), value.to_string());
    }
    map
}
