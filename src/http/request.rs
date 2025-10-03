use std::{
    collections::HashMap,
    io::{BufReader, Read},
};

use crate::{
    http::{http_trait::Http, url::URL},
    utils::{self, proxy::HEADER_PROXY_DESTINATION},
};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub destination: URL,
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    fn get_destination(headers: &HashMap<String, String>) -> Result<URL, String> {
        if let Some(destination) = headers.get(HEADER_PROXY_DESTINATION) {
            return Ok(URL::new(destination)?);
        }

        let config = utils::config::get_config();
        if let Some(ref destination) = config.default_destination {
            return Ok(URL::new(destination)?);
        }

        return Err("No destination found".into());
    }

    pub fn parse<R: Read>(stream: &mut R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(stream);

        let (method, path, version) = Self::parse_first_line(&mut reader)?;

        let mut headers = Self::parse_headers(&mut reader)?;
        let destination = HttpRequest::get_destination(&headers)?;
        headers.remove(HEADER_PROXY_DESTINATION);
        headers.insert("host".to_string(), destination.hostname.clone());
        // TODO: handle keep-alive between proxy and destination
        headers.insert("connection".to_string(), "close".to_string());

        let request = Self {
            method,
            path,
            version,
            headers,
            destination,
            body: None,
        };
        return Ok(request);
    }
}

impl Http for HttpRequest {
    fn get_first_line(&self) -> String {
        return format!("{} {} {}", self.method, self.path, self.version);
    }

    fn get_headers(&self) -> HashMap<String, String> {
        return self.headers.clone();
    }

    fn get_body(&self) -> Option<Vec<u8>> {
        return self.body.clone();
    }
}
