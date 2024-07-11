

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