use crate::{error::XsusError, response::Response};

pub fn parse_response(raw: &str) -> Result<Response, XsusError> {
    let mut lines = raw.lines();
    let status_lines = lines
        .next()
        .ok_or(XsusError::Parse("Empty response".into()))?;
}
