use xsus::Xsus;

fn main() {
    let mut client = Xsus::new("http://example.com");
    client.interceptors.request.push(Box::new(|mut req| {
        req.headers.insert(
            "X-Custom-Auth".to_string(),
            "Xsus-Security-Token".to_string(),
        );
        req
    }));

    println!("Sending Xsus Request !...");

    match client.get("/") {
        Ok(res) => {
            println!("Status: {}", res.status);
            println!("Headers: {:?}", res.headers);
            println!("Data Snippet: {}", &res.data[..50]);
        }
        Err(e) => eprintln!("Xsus Error: {}", e),
    }
}
