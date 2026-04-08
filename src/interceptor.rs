use crate::{request::Request, response::Response};

pub type RequestInterceptor = Box<dyn Fn(Request) -> Request>;
pub type ResponseInterceptor = Box<dyn Fn(Response) -> Response>;

pub struct Interceptors {
    pub request: Vec<RequestInterceptor>,
    pub response: Vec<ResponseInterceptor>,
}
