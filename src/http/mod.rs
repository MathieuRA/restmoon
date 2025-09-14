use std::error::Error;

pub mod request;

#[derive(Debug)]
pub struct URL {
    pub protocol: String,
    pub hostname: String,
    pub path: String,
    pub port: u16, // maybe more/less?
}

impl URL {
    pub fn new(string: &str) -> Result<Self, String> {
        let mut parts: Vec<&str> = string.split("://").collect();
        if parts.len() != 2 {
            return Err(format!("Invalid URL {}", string).into());
        }

        let protocol = parts[0].to_string();
        let mut rest = parts[1];

        parts = rest.split(":").collect();
        let port: u16 = if parts.len() == 2 {
            match parts[0].parse() {
                Ok(port) => port,
                Err(_) => {
                    // TODO: need to fix, seems to broke with simple port like 1234
                    return Err(format!("Invalid port {}", string));
                }
            }
        } else {
            if protocol.eq("https") { 443 } else { 80 }
        };

        rest = parts.last().unwrap();
        parts = rest.split("/").collect();

        let hostname = parts[0].to_string();
        let path = parts[1].to_string();

        return Ok(Self {
            port,
            protocol,
            hostname,
            path,
        });
    }

    pub fn to_string(self) -> String {
        return String::from(format!(
            "{}://{}/{}",
            self.protocol, self.hostname, self.path,
        ));
    }
}
