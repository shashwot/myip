use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;

fn main() {
    match fetch_public_ip() {
        Ok(ip) => println!("{}", ip),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn fetch_public_ip() -> Result<IpAddr, String> {
    let addr = "api.ipify.org:80"
        .parse()
        .or_else(|_| {
            use std::net::ToSocketAddrs;
            "api.ipify.org:80"
                .to_socket_addrs()
                .map_err(|e| e.to_string())?
                .next()
                .ok_or_else(|| "DNS resolution failed".to_string())
        })
        .map_err(|e: String| e)?;

    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5))
        .map_err(|e| format!("Connection failed: {}", e))?;

    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    stream.set_write_timeout(Some(Duration::from_secs(5))).ok();

    let request = "GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n";
    stream
        .write_all(request.as_bytes())
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let mut response = Vec::with_capacity(512);
    stream
        .take(1024)
        .read_to_end(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let response_str = String::from_utf8_lossy(&response);

    let first_line = response_str.lines().next().unwrap_or("");
    if !first_line.contains("200") {
        return Err(format!("Unexpected HTTP status: {}", first_line));
    }

    let body = response_str
        .find("\r\n\r\n")
        .map(|i| response_str[i + 4..].trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or("Empty or missing response body")?;

    IpAddr::from_str(&body).map_err(|_| format!("Response is not a valid IP: {:?}", body))
}
