use serde_json::Value;


pub async fn get_next_cursor(body_data: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let cursor_entry: &Value = body_data["timeline"]["instructions"][0]["addEntries"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .find(|entry| {
            if let Some(entry_id) = entry["entryId"].as_str() {
                entry_id == "sq-cursor-bottom"
            } else {
                false
            }
        })
        .unwrap();
    let cursor = cursor_entry["content"]["operation"]["cursor"]["value"]
        .as_str()
        .unwrap()
        .to_string();
    Ok(cursor)
}
