use std::collections::HashMap;

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}
