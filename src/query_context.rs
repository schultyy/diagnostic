use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct QueryContext {
  working_directory: PathBuf
}

impl QueryContext {
  pub fn new(working_directory: &str) -> Result<QueryContext, String> {
    let srcdir = PathBuf::from(working_directory);
    let working_dir = match fs::canonicalize(&srcdir) {
      Ok(p) => p,
      Err(_) => return Err(format!("Failed to get absolute path from {:?}", working_directory))
    };

    Ok(QueryContext {
      working_directory: working_dir
    })
  }

  pub fn execute_query(&self, query: String) -> Result<String, String> {
    Err("Failed to execute query".into())
  }
}