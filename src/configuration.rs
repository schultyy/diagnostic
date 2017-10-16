use std::fs::{self, File};
use std::io::prelude::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Column {
  pub name: String,
  pub regex: String,
  pub capture_group: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
  pub file_type: String,
  pub columns: Vec<Column>
}

impl Configuration {
  pub fn from_file(filename: &str) -> Configuration {
    let absolute_path = fs::canonicalize(filename).unwrap();
    let mut file = File::open(absolute_path).unwrap();
    let mut serialized = String::new();
    file.read_to_string(&mut serialized);

    serde_json::from_str(&serialized).unwrap()
  }
}