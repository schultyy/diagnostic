pub struct QueryRequest {
    pub log_filename: String,
    pub query_fields: Vec<String>,
    pub conditional: Option<QueryClause>,
}

impl QueryRequest {
    pub fn new(log_filename: String, query_fields: Vec<String>, conditional_field: Option<QueryClause>) -> QueryRequest {
        QueryRequest {
            log_filename: log_filename,
            query_fields: query_fields,
            conditional: conditional_field
        }
    }
}

pub struct QueryClause {
    pub conditional_field: String,
    pub conditional_value: String
}

pub struct QueryRequestBuilder {
    pub log_filename: Option<String>,
    pub query_fields: Option<Vec<String>>,
    pub conditional: Option<QueryClause>
}

impl QueryRequestBuilder {
    pub fn new() -> QueryRequestBuilder {
        QueryRequestBuilder {
            log_filename: None,
            query_fields: None,
            conditional: None
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

    pub fn set_conditional_field(&mut self, conditional_field: String, conditional_value: String) -> &mut Self {
        self.conditional = Some(QueryClause{
            conditional_field: conditional_field,
            conditional_value: conditional_value
        });

        self
    }

    pub fn build(self) -> QueryRequest {
        QueryRequest::new(
            self.log_filename.expect("log filename must be present"),
            self.query_fields.expect("query fields must be present"),
            self.conditional
        )
    }
}