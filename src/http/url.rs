use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct URL {
    pub protocol: String,
    pub hostname: String,
    pub path: Option<String>,
    pub port: u16,
}

impl URL {
    pub fn new(string: &str) -> Result<Self, String> {
        let (protocol, rest) = string
            .split_once("://")
            .ok_or_else(|| format!("Missing protocol in URL: {}", string))?;

        let default_port = match protocol {
            "http" => 80,
            "https" => 443,
            _ => return Err(format!("Unsupported protocol: {}", protocol)),
        };

        let (host_port, path) = match rest.split_once('/') {
            Some((host_port, path_part)) => (host_port, Some(format!("/{}", path_part))),
            None => (rest, None),
        };

        let (hostname, port) = match host_port.split_once(':') {
            Some((host, port_str)) => {
                let port = port_str
                    .parse::<u16>()
                    .map_err(|_| format!("Invalid port '{}' in URL: {}", port_str, string))?;
                (host.to_string(), port)
            }
            None => (host_port.to_string(), default_port),
        };

        if hostname.is_empty() {
            return Err(format!("Empty hostname in URL: {}", string));
        }

        Ok(Self {
            protocol: protocol.to_string(),
            hostname,
            port,
            path,
        })
    }

    pub fn to_string(self) -> String {
        let path = match self.path {
            Some(path) => format!("!{}", path),
            None => "".to_string(),
        };

        return String::from(format!(
            "{}://{}:{}{}",
            self.protocol, self.hostname, self.port, path,
        ));
    }
}

impl FromStr for URL {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::new(str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_parsing() {
        let url = URL::new("https://localhost:8080/test").unwrap();
        assert_eq!(url.protocol, "https");
        assert_eq!(url.hostname, "localhost");
        assert_eq!(url.port, 8080);
        assert_eq!(url.path, Some("/test".to_string()));

        let url = URL::new("https://localhost/v1/test").unwrap();
        assert_eq!(url.port, 443);
        assert_eq!(url.path, Some("/v1/test".to_string()));

        let url = URL::new("http://localhost").unwrap();
        assert_eq!(url.port, 80);
        assert_eq!(url.path, None);

        assert!(URL::new("invalid-url").is_err());
        assert!(URL::new("https://example.com:abc").is_err());
    }

    #[test]
    fn test_from_str() {
        let url: URL = "https://api.localhost:9000/v1/test-str".parse().unwrap();
        assert_eq!(url.protocol, "https");
        assert_eq!(url.hostname, "api.localhost");
        assert_eq!(url.port, 9000);
        assert_eq!(url.path, Some("/v1/test-str".to_string()));
    }
}
