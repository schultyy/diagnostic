use std::fs;
use std::path::PathBuf;
use configuration::Configuration;
use query_request::{QueryRequest, QueryRequestBuilder};
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
        let mut request_builder = QueryRequestBuilder::new();

        let left_node = match query_ast.left {
            Some(n) => n,
            None => return Err("Expected Log file node, got None".into())
        };

        let optional_right_node = match query_ast.right {
            Some(n) => Some(n),
            None => None
        };

        if let log_ql::parser::GrammarItem::LogFile { ref fields, ref filename } = left_node.entry {
            request_builder.set_log_filename(filename.clone());
            request_builder.set_query_fields(fields.clone());
        } else {
            return Err("Couldn't deref Logfile node".into());
        }

        if let Some(right_node) = optional_right_node {
            if let Some(condition_node) = right_node.clone().left {
                if let log_ql::parser::GrammarItem::Condition { ref field, ref value } = condition_node.entry {
                    request_builder.set_conditional_field(field.clone(), value.clone());
                }
            }

            if let Some(limit_node) = right_node.clone().right {
                if let log_ql::parser::GrammarItem::Limit { ref number_of_rows, ref direction } = limit_node.entry {
                    request_builder.set_limit_clause(number_of_rows.clone(), direction.clone());
                }
            }
        }

        Ok(request_builder.build())
    }

    fn filter_log_file(&self, log_file: log_file::LogFile, query_request: QueryRequest) -> Vec<String> {
        let filtered_log_results = log_file
            .search_field(&query_request.conditional);

        let limited_log_results;

        if let Some(limit_clause) = query_request.limit.clone() {
            limited_log_results = filtered_log_results
                .iter()
                .take(limit_clause.number_of_rows)
                .map(|r| r.clone())
                .collect::<Vec<log_file::Row>>();
        } else {
            limited_log_results = filtered_log_results;
        }

        limited_log_results
            .iter()
            .flat_map(|r| {
                query_request.query_fields
                    .iter()
                    .map(|query_field| r.get_field(query_field))
                    .collect::<Option<String>>()
            })
            .collect::<Vec<String>>()
    }

    pub fn execute_query(&self, query: String) -> Result<Vec<String>, String> {
        let query_request = self.parse(query)?;

        let log_file = log_file::from_file(self.working_directory.join(&query_request.log_filename), &self.configuration);
        Ok(self.filter_log_file(log_file, query_request))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use configuration;

    fn build_query_context() -> QueryContext {
        let configuration = configuration::Configuration::from_file("./support/config.json");
        QueryContext::new("./support", configuration).unwrap()
    }

    #[test]
    fn query_with_where_clause_returns_exactly_filtered_rows() {
        let query = "SELECT severity FROM 'travis.log' WHERE severity = 'INFO'".into();
        let query_context = build_query_context();
        let result_set = query_context.execute_query(query).unwrap();
        assert_eq!(result_set.len(), 20);
    }

    #[test]
    fn query_with_limit_10_returns_exactly_10_lines() {
        let query = "SELECT component FROM 'travis.log' LIMIT 10".into();
        let query_context = build_query_context();
        let result_set = query_context.execute_query(query).unwrap();
        assert_eq!(result_set.len(), 10);
    }
}