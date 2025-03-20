use std::{fs::File, io::BufReader};

use base64::prelude::*;

pub fn get_local_json_data(path: &str) -> serde_json::Map<String, serde_json::Value> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let json_data: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let input: &serde_json::Map<String, serde_json::Value> = json_data.as_object().unwrap();
    input.clone()
}

pub fn encode_base64(text: &str) -> String {
    BASE64_STANDARD.encode(text.as_bytes())
}
