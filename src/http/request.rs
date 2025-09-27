use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    // body: Vec<u8>,
}

impl HttpRequest {
    pub fn parse(stream: &mut TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(stream);

        // GET /vms HTTP/1.1\r\n
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid HTTP request line".into());
        }

        let method = parts[0].to_string();
        let path = parts[1].to_string();
        let version = parts[2].to_string();

        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line = line.trim();

            if line.is_empty() {
                break;
            }

            if let Some(position) = line.find(":") {
                let key = line[..position].trim().to_lowercase();
                let value = line[position + 1..].trim().to_lowercase();
                headers.insert(key, value);
            }
        }
        // TODO: handle keep-alive between proxy and destination
        headers.insert("connection".to_string(), "close".to_string());

        let request = Self {
            method,
            path,
            version,
            headers,
        };
        return Ok(request);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        let request_line = format!("{} {} {}\r\n", self.method, self.path, self.version);
        result.extend_from_slice(request_line.as_bytes());

        for (key, value) in &self.headers {
            let header_line = format!("{}: {}\r\n", key, value);
            result.extend_from_slice(header_line.as_bytes());
        }
        result.extend_from_slice("\r\n".as_bytes()); // end of headers

        return result;
    }
}
