use std::collections::HashMap;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub header: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new(method: Method, url: &str) -> Self {}
}
