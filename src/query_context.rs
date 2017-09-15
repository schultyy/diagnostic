use std::fs;
use std::path::PathBuf;
use configuration::Configuration;
use query_request::QueryRequest;
use log_file;
use log_ql;

pub struct QueryContext {
    working_directory: PathBuf,
    configuration: Configuration
}

impl QueryContext {
    pub fn new(working_directory: &str, configuration: Configuration) -> Result<QueryContext, String> {
        let src_dir = PathBuf::from(working_directory);
        let working_dir = match fs::canonicalize(&src_dir) {
            Ok(p) => p,
            Err(_) => return Err(format!("Failed to get absolute path from {:?}", working_directory))
        };

        Ok(QueryContext {
            working_directory: working_dir,
            configuration: configuration
        })
    }

    fn parse(&self, query: String) -> Result<QueryRequest, String> {
        let mut parser = log_ql::parser::Parser::new(query);
        let query_ast = parser.parse()?;
        let log_filename;
        let query_fields;
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

        if let log_ql::parser::GrammarItem::LogFile { ref fields, ref filename } = left_node.entry {
            log_filename = filename.clone();
            query_fields = fields.clone();
        } else {
            return Err("Couldn't deref Logfile node".into());
        }

        if let log_ql::parser::GrammarItem::Condition { ref field, ref value } = right_node.entry {
            conditional_field = field.clone();
            conditional_value = value.clone();
        } else {
            return Err("Couldn't deref Conditional node".into());
        }

        Ok(QueryRequest::new(log_filename, query_fields, conditional_field, conditional_value))
    }

    fn filter_log_file(&self, log_file: log_file::LogFile, query_fields: Vec<String>, conditional_field: String, conditional_value: String) -> Vec<String> {
        log_file
            .search_field(conditional_field, conditional_value)
            .iter()
            .flat_map(|r| {
                query_fields
                    .iter()
                    .map(|query_field| r.get_field(query_field))
                    .collect::<Option<String>>()
            })
            .collect::<Vec<String>>()
    }

    pub fn execute_query(&self, query: String) -> Result<Vec<String>, String> {
        let query_request = self.parse(query)?;

        let log_file = log_file::from_file(self.working_directory.join(query_request.log_filename), &self.configuration);
        Ok(self.filter_log_file(log_file, query_request.query_fields, query_request.conditional_field, query_request.conditional_value))
    }
}