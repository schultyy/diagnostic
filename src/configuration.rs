use std::fs::File;
use std::io::prelude::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
  pub file_type: String,
  pub columns: Vec<String>,
  pub regex: String
}

impl Configuration {
  pub fn from_file(filename: &str) -> Configuration {
    let mut file = File::open(filename).unwrap();
    let mut serialized = String::new();
    file.read_to_string(&mut serialized);

    serde_json::from_str(&serialized).unwrap()
  }
}