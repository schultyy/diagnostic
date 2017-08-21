use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

use configuration::Configuration;

pub struct LogFile {

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

fn parse_line(line: &str, config: &Configuration) {
  for column in &config.columns {
    let result = execute_regex(line, column.regex.clone(), column.capture_group);
    println!("{} - {:?}", column.name, result);
  }
}

pub fn from_file(filename: PathBuf, config: &Configuration) -> LogFile {
  let mut file = File::open(filename).unwrap();
  let mut complete_file = String::new();
  file.read_to_string(&mut complete_file);

  for line in complete_file.split("\n") {
    parse_line(line, &config);
  }

  LogFile { }
}