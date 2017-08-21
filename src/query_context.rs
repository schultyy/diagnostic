use std::fs;
use std::path::PathBuf;
use configuration::Configuration;
use log_file;

use log_ql;

pub struct QueryContext {
  working_directory: PathBuf,
  configuration: Configuration
}

impl QueryContext {
  pub fn new(working_directory: &str, configuration: Configuration) -> Result<QueryContext, String> {
    let srcdir = PathBuf::from(working_directory);
    let working_dir = match fs::canonicalize(&srcdir) {
      Ok(p) => p,
      Err(_) => return Err(format!("Failed to get absolute path from {:?}", working_directory))
    };

    Ok(QueryContext {
      working_directory: working_dir,
      configuration: configuration
    })
  }

  fn parse(&self, query: String) -> Result<(String, String, String, String), String> {
    let query_ast = try!(log_ql::get_ast_for_query(query));
    let log_filename;
    let query_field;
    let conditional_field;
    let conditional_value;

    let left_node = match query_ast.left {
      Some(n) => n,
      None => return Err("Expected Log file node, got None".into())
    };

    let right_node = match query_ast.right {
      Some(n) => n,
      None => return Err("Expected conditional query node, got None".into())
    };

    if let log_ql::GrammarItem::LogFile { ref field, ref filename } = left_node.entry {
      log_filename = filename.clone();
      query_field = field.clone();
    } else {
      return Err("Couldn't deref Logfile node".into());
    }

    if let log_ql::GrammarItem::Condition { ref field, ref value } = right_node.entry {
      conditional_field = field.clone();
      conditional_value = value.clone();
    } else {
      return Err("Couldn't deref Conditional node".into());
    }

    Ok((log_filename, query_field, conditional_field, conditional_value))
  }

  pub fn execute_query(&self, query: String) -> Result<String, String> {
    let (log_filename, query_field, conditional_field, conditional_value) = try!(self.parse(query));

    log_file::from_file(self.working_directory.join(log_filename), &self.configuration);

    Err("Failed to execute query".into())
  }
}