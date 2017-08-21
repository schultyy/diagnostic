use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

use configuration::Configuration;

#[derive(Debug)]
#[derive(Clone)]
pub struct Row {
  values: HashMap<String, String>
}

#[derive(Debug)]
pub struct LogFile {
  rows: Vec<Row>
}

impl Row {
  pub fn new() -> Row {
    Row {
      values: HashMap::new()
    }
  }

  pub fn get_field(&self, key: &String) -> Option<String> {
    match self.values.get(key) {
      Some(value) => Some(value.clone()),
      None => None
    }
  }

  pub fn store(&mut self, key: String, value: String) {
    self.values.insert(key, value);
  }

  pub fn matches(&self, key: &String, expected_value: &String) -> bool {
    match self.values.get(key) {
      Some(actual_value) => actual_value == expected_value,
      None => false
    }
  }
}

impl LogFile {
  pub fn new() -> LogFile {
    LogFile {
      rows: vec!()
    }
  }

  pub fn store(&mut self, row: Row) {
    self.rows.push(row);
  }

  pub fn search_field(&self, field_name: String, field_value: String) -> Vec<Row> {
    self.rows.iter()
              .filter(|r| r.matches(&field_name, &field_value))
              .map(|r| r.clone())
              .collect()
  }
}

fn execute_regex(line: &str, regex: String, capture_group: usize) -> Option<String> {
  let compiled_regex = Regex::new(&regex).unwrap();
  let mut captures = compiled_regex.captures_iter(line);
  captures
    .nth(0)
    .and_then(|capture| capture.get(capture_group))
    .and_then(|m| Some(m.as_str()))
    .and_then(|m| Some(m.to_string()))
}

fn parse_line(log_file: &mut LogFile, line: &str, config: &Configuration) {
  let mut row = Row::new();

  for column in &config.columns {
    if let Some(regex_result) = execute_regex(line, column.regex.clone(), column.capture_group) {
      row.store(column.name.clone(), regex_result);
    }
  }
  log_file.store(row);
}

pub fn from_file(filename: PathBuf, config: &Configuration) -> LogFile {
  let mut file = File::open(filename).unwrap();
  let mut complete_file = String::new();
  file.read_to_string(&mut complete_file);
  let mut log_file = LogFile::new();

  for line in complete_file.split("\n") {
    parse_line(&mut log_file, line, &config);
  }

  log_file
}