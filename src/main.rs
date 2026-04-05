use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(10);
const DNS_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_RESPONSE_BYTES: usize = 4096;
const HOST_V4: &str = "api4.ipify.org";
const HOST_V6: &str = "api6.ipify.org";
const USER_AGENT: &str = "rust-ip-fetcher/1.0";

#[derive(Debug)]
pub enum Error {
    Dns(String),
    Connect(String),
    Timeout(String),
    Http(String),
    Io(String),
    Utf8(String),
    InvalidIp(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Dns(msg) => write!(f, "DNS error: {}", msg),
            Error::Connect(msg) => write!(f, "Connection error: {}", msg),
            Error::Timeout(msg) => write!(f, "Timeout: {}", msg),
            Error::Http(msg) => write!(f, "HTTP error: {}", msg),
            Error::Io(msg) => write!(f, "I/O error: {}", msg),
            Error::Utf8(msg) => write!(f, "UTF-8 error: {}", msg),
            Error::InvalidIp(msg) => write!(f, "Invalid IP: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;


fn main() {
    let (tx4, rx4) = mpsc::channel();
    let (tx6, rx6) = mpsc::channel();

    thread::spawn(move || {
        let _ = tx4.send(fetch_ip(HOST_V4, AddressFamily::V4));
    });
    thread::spawn(move || {
        let _ = tx6.send(fetch_ip(HOST_V6, AddressFamily::V6));
    });

    let v4 = rx4.recv().unwrap_or(Err(Error::Timeout("v4 thread died".into())));
    let v6 = rx6.recv().unwrap_or(Err(Error::Timeout("v6 thread died".into())));

    match (v4, v6) {
        (Ok(v4), Ok(v6)) => println!("{}, {}", v4, v6),
        (Ok(v4), Err(e)) => {
            eprintln!("IPv6 unavailable: {}", e);
            println!("{}", v4);
        }
        (Err(e), Ok(v6)) => {
            eprintln!("IPv4 unavailable: {}", e);
            println!("{}", v6);
        }
        (Err(e4), Err(e6)) => {
            eprintln!("IPv4 error: {}", e4);
            eprintln!("IPv6 error: {}", e6);
            std::process::exit(1);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum AddressFamily {
    V4,
    V6,
}

fn fetch_ip(host: &str, family: AddressFamily) -> Result<IpAddr> {
    let addrs = resolve_with_timeout(host, family, DNS_TIMEOUT)?;

    let mut last_err = None;
    for addr in addrs {
        match connect_and_fetch(&addr, host) {
            Ok(ip) => return Ok(ip),
            Err(e) => {
                last_err = Some(e);
                continue;
            }
        }
    }
    Err(last_err.unwrap_or_else(|| Error::Connect("no usable address".into())))
}

fn resolve_with_timeout(
    host: &str,
    family: AddressFamily,
    timeout: Duration,
) -> Result<Vec<SocketAddr>> {
    let host_port = format!("{}:80", host);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = host_port.to_socket_addrs().map(|iter| iter.collect::<Vec<_>>());
        let _ = tx.send(result);
    });

    let all_addrs = rx
        .recv_timeout(timeout)
        .map_err(|_| Error::Timeout(format!("DNS lookup for {}", host)))?
        .map_err(|e| Error::Dns(format!("{}: {}", host, e)))?;

    let filtered: Vec<SocketAddr> = all_addrs
        .into_iter()
        .filter(|addr| match family {
            AddressFamily::V4 => addr.is_ipv4(),
            AddressFamily::V6 => addr.is_ipv6(),
        })
        .collect();

    if filtered.is_empty() {
        Err(Error::Dns(format!(
            "no {} addresses found for {}",
            if family == AddressFamily::V4 { "IPv4" } else { "IPv6" },
            host
        )))
    } else {
        Ok(filtered)
    }
}

fn connect_and_fetch(addr: &SocketAddr, host: &str) -> Result<IpAddr> {
    let mut stream = TcpStream::connect_timeout(addr, TIMEOUT)
        .map_err(|e| Error::Connect(format!("{}: {}", addr, e)))?;

    stream
        .set_read_timeout(Some(TIMEOUT))
        .map_err(|e| Error::Io(e.to_string()))?;
    stream
        .set_write_timeout(Some(TIMEOUT))
        .map_err(|e| Error::Io(e.to_string()))?;

    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nConnection: close\r\n\r\n",
        host, USER_AGENT
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|e| Error::Io(format!("write: {}", e)))?;
    stream.flush().map_err(|e| Error::Io(format!("flush: {}", e)))?;

    let mut response = Vec::with_capacity(MAX_RESPONSE_BYTES);
    let mut take_stream = stream.take(MAX_RESPONSE_BYTES as u64);
    take_stream
        .read_to_end(&mut response)
        .map_err(|e| Error::Io(format!("read: {}", e)))?;

    let body = parse_http_response(&response)?;
    let trimmed = body.trim();

    IpAddr::from_str(trimmed)
        .map_err(|_| Error::InvalidIp(trimmed.to_string()))
}


fn parse_http_response(raw: &[u8]) -> Result<&str> {
    let text = std::str::from_utf8(raw).map_err(|e| Error::Utf8(e.to_string()))?;

    let header_end = text
        .find("\r\n\r\n")
        .or_else(|| text.find("\n\n"))
        .ok_or_else(|| Error::Http("missing header/body separator".into()))?;

    let headers = &text[..header_end];
    let body = &text[header_end + if text[header_end..].starts_with("\r\n\r\n") { 4 } else { 2 }..];

    let status_line = headers
        .lines()
        .next()
        .ok_or_else(|| Error::Http("empty headers".into()))?;

    if !(status_line.starts_with("HTTP/1.") && status_line.contains("200")) {
        return Err(Error::Http(format!("unexpected status: {}", status_line)));
    }

    Ok(body)
}