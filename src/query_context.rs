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

        if let Some(where_and_limit_node) = optional_right_node {

            if let &Some(ref condition_node) = &where_and_limit_node.left {
                if let log_ql::parser::GrammarItem::Condition { ref field, ref value } = condition_node.entry {
                    request_builder.set_conditional_field(field.clone(), value.clone());
                }
            }

            if let Some(limit_node) = where_and_limit_node.right {
                if let log_ql::parser::GrammarItem::Limit { number_of_rows, direction } = limit_node.entry {
                    request_builder.set_limit_field(number_of_rows, direction);
                }
            }
        }

        Ok(request_builder.build())
    }

    fn limit_search_results(&self, search_results: Vec<log_file::Row>, query_request: &QueryRequest) -> Vec<log_file::Row> {

        if search_results.len() == 0 {
            return search_results
        }

        if let &Some(ref limit_clause) = &query_request.limit_clause {

            if limit_clause.number_of_rows > search_results.len() {
                return search_results
            }

            if limit_clause.direction == log_ql::parser::LimitDirection::First {
                search_results
                    .iter()
                    .take(limit_clause.number_of_rows)
                    .map(|r| r.clone())
                    .collect::<Vec<log_file::Row>>()
            } else {
                search_results
                    .iter()
                    .skip(search_results.len() - limit_clause.number_of_rows - 1)
                    .take(limit_clause.number_of_rows)
                    .map(|r| r.clone())
                    .collect::<Vec<log_file::Row>>()
            }
        } else {
            search_results
        }
    }

    fn filter_log_file(&self, log_file: log_file::LogFile, query_request: QueryRequest) -> Vec<String> {
        let all_search_results = log_file.search_field(&query_request.conditional);

        let limited_search_results = self.limit_search_results(all_search_results, &query_request);

        limited_search_results.iter().flat_map(|r| {
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
    use configuration;
    use super::*;

    fn load_configuration() -> configuration::Configuration {
        configuration::Configuration::from_file("./test_support/config.json")
    }

    fn build_query_context() -> QueryContext {
        QueryContext::new("./test_support", load_configuration()).unwrap()
    }

    #[test]
    fn test_query_with_limit_10_clause() {
        let query_context = build_query_context();
        let results = query_context.execute_query("SELECT date FROM 'travis.log' LIMIT 10".into()).unwrap();
        assert_eq!(results.len(), 10);
    }

    #[test]
    fn test_query_with_limit_last_1_clause() {
        let query_context = build_query_context();
        let results = query_context.execute_query("SELECT date FROM 'travis.log' LIMIT LAST 1".into()).unwrap();
        assert_eq!(results[0], "[2017-08-20T20:21:49+0000]");
    }

    #[test]
    fn test_limit_last_n_rows_to_max_number_of_result_set() {
        /*
            Given the case a result set has 100 rows and the user tries to limit to the last 500 rows.
            We then can only return the full result set
        */
        let query_context = build_query_context();
        let results = query_context.execute_query("SELECT date FROM 'travis.log' LIMIT LAST 15000".into()).unwrap();
        assert_eq!(results.len(), 100);
    }
}