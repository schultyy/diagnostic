pub struct QueryRequest {
    pub log_filename: String,
    pub query_fields: Vec<String>,
    pub conditional_field: Option<String>,
    pub conditional_value: Option<String>
}

impl QueryRequest {
    pub fn new(log_filename: String, query_fields: Vec<String>, conditional_field: Option<String>, conditional_value: Option<String>) -> QueryRequest {
        QueryRequest {
            log_filename: log_filename,
            query_fields: query_fields,
            conditional_field: conditional_field,
            conditional_value: conditional_value
        }
    }
}

pub struct QueryRequestBuilder {
    pub log_filename: Option<String>,
    pub query_fields: Option<Vec<String>>,
    pub conditional_field: Option<String>,
    pub conditional_value: Option<String>
}

impl QueryRequestBuilder {
    pub fn new() -> QueryRequestBuilder {
        QueryRequestBuilder {
            log_filename: None,
            query_fields: None,
            conditional_field: None,
            conditional_value: None
        }
    }

    pub fn set_log_filename(&mut self, log_filename: String) -> &mut Self {
        self.log_filename = Some(log_filename);
        self
    }

    pub fn set_query_fields(&mut self, query_fields: Vec<String>) -> &mut Self {
        self.query_fields = Some(query_fields);
        self
    }

    pub fn set_conditional_field(&mut self, conditional_field: String) -> &mut Self {
        self.conditional_field = Some(conditional_field);
        self
    }

    pub fn set_conditional_value(&mut self, conditional_value: String) -> &mut Self {
        self.conditional_value = Some(conditional_value);
        self
    }

    pub fn build(self) -> QueryRequest {
        QueryRequest::new(
            self.log_filename.expect("log filename must be present"),
            self.query_fields.expect("query fields must be present"),
            self.conditional_field,
            self.conditional_value
        )
    }
}