pub struct ParsedUrl {
    pub host: String,
    pub port: String,
    pub path: String,
}

pub fn parse_url(url: &str) -> Result<ParsedUrl, String> {
    let remainder = url
        .strip_prefix("http://")
        .ok_or("Only Http is supported")?;
    let (host_port, path_query) = remainder.split_once('/').unwrap_or((remainder, ""));
    let (host, port) = if let Some((h, p)) = host_port.split_once(':') {
        (h.to_string(), p.to_string())
    } else {
        (host_port.to_string(), "80".to_string())
    };
}
