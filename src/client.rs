use crate::{
    error::XsusError,
    interceptor::Interceptors,
    parser::parse_response,
    request::{Method, Request},
    response::Response,
    transport::execute_network_call,
};
use std::time::Duration;

pub struct Xsus {
    pub base_url: String,
    pub timeout: Duration,
    pub interceptors: Interceptors,
}

impl Xsus {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            timeout: Duration::from_secs(10),
            interceptors: Interceptors::new(),
        }
    }

    pub fn get(&self, path: &str) -> Result<Response, XsusError> {
        let base = self.base_url.trim_end_matches('/');
        let sub = path.trim_start_matches('/');
        let full_url = format!("{}/{}", base, sub);

        let mut req = Request::new(Method::GET, &full_url);

        for interceptor in &self.interceptors.request {
            req = interceptor(req);
        }

        let raw_res = execute_network_call(&req, self.timeout)?;
        let mut res = parse_response(&raw_res)?;

        for interceptor in &self.interceptors.response {
            res = interceptor(res);
        }
        Ok(res)
    }
}
