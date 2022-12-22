use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Prog {
    pub functions: Vec<serde_json::Value>,
}

pub fn load_prog(path: &str) -> Prog {
    let data = fs::File::open(path).expect("Error loading file.");
    let value = serde_json::from_reader(data).expect("Error parsing file.");
    serde_json::from_value(value).unwrap()
}
