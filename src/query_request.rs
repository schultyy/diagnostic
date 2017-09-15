pub struct QueryRequest {
    pub log_filename: String,
    pub query_fields: Vec<String>,
    pub conditional_field: String,
    pub conditional_value: String
}

impl QueryRequest {
    pub fn new(log_filename: String, query_fields: Vec<String>, conditional_field: String, conditional_value: String) -> QueryRequest {
        QueryRequest {
            log_filename: log_filename,
            query_fields: query_fields,
            conditional_field: conditional_field,
            conditional_value: conditional_value
        }
    }
}