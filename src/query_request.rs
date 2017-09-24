use log_ql;

pub struct QueryRequest {
    pub log_filename: String,
    pub query_fields: Vec<String>,
    pub conditional: Option<QueryClause>,
    pub limit: Option<LimitClause>
}

impl QueryRequest {
    pub fn new(log_filename: String, query_fields: Vec<String>, conditional_field: Option<QueryClause>, limit: Option<LimitClause>) -> QueryRequest {
        QueryRequest {
            log_filename: log_filename,
            query_fields: query_fields,
            conditional: conditional_field,
            limit: limit
        }
    }
}

pub struct QueryClause {
    pub conditional_field: String,
    pub conditional_value: String
}

pub struct LimitClause {
    pub number_of_rows: usize,
    pub direction: log_ql::parser::LimitDirection
}

pub struct QueryRequestBuilder {
    pub log_filename: Option<String>,
    pub query_fields: Option<Vec<String>>,
    pub conditional: Option<QueryClause>,
    pub limit: Option<LimitClause>
}

impl QueryRequestBuilder {
    pub fn new() -> QueryRequestBuilder {
        QueryRequestBuilder {
            log_filename: None,
            query_fields: None,
            conditional: None,
            limit: None
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

    pub fn set_limit_clause(&mut self, number_of_rows: usize, direction: log_ql::parser::LimitDirection) -> &mut Self {
        self.limit = Some(LimitClause{
            number_of_rows: number_of_rows,
            direction: direction
        });

        self
    }

    pub fn build(self) -> QueryRequest {
        QueryRequest::new(
            self.log_filename.expect("log filename must be present"),
            self.query_fields.expect("query fields must be present"),
            self.conditional,
            self.limit
        )
    }
}