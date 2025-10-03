use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader, Read},
};

use crate::http::http_trait::Http;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: String,
    pub headers: HashMap<String, String>,
    pub code: u16,
    pub reason: String,
    pub body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn parse<R: Read>(mut reader: R) -> Result<Self, Box<dyn Error>> {
        let mut buf_reader = BufReader::new(&mut reader);

        let (version, code, reason) = Self::parse_first_line(&mut buf_reader)?;

        let headers = Self::parse_headers(&mut buf_reader)?;
        let mut is_chunked = false;
        if let Some(chunked) = headers.get("transfer-encoding") {
            if chunked.to_lowercase().contains("chunked") {
                is_chunked = true;
            }
        }
        let body = if is_chunked {
            Self::decode_chunked(&mut buf_reader)?
        } else {
            let mut body = Vec::new();
            buf_reader.read_to_end(&mut body)?;
            body
        };

        Ok(Self {
            version,
            code: code.parse::<u16>()?,
            reason,
            headers,
            body: Some(body),
        })
    }

    fn decode_chunked<R: BufRead>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut body: Vec<u8> = Vec::new();

        loop {
            let mut size_line = String::new();
            reader.read_line(&mut size_line)?;
            let size_hex = size_line.trim();

            let chunk_size = usize::from_str_radix(size_hex, 16)
                .map_err(|_| format!("Invalid chunk size: '{}'", size_hex))?;

            if chunk_size == 0 {
                break;
            }

            let mut chunk_data = vec![0; chunk_size];
            reader.read_exact(&mut chunk_data)?;
            body.extend_from_slice(&chunk_data);

            let mut crlf = [0; 2];
            reader.read_exact(&mut crlf)?;

            if &crlf != b"\r\n" {
                return Err(format!("Expected CRLF after chunk, got {:?}", crlf).into());
            }
        }

        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if line.trim().is_empty() {
                break;
            }
        }

        return Ok(body);
    }
}

impl Http for HttpResponse {
    fn get_first_line(&self) -> String {
        return format!("{} {} {}", self.version, self.code, self.reason);
    }

    fn get_headers(&self) -> HashMap<String, String> {
        return self.headers.clone();
    }

    fn get_body(&self) -> Option<Vec<u8>> {
        return self.body.clone();
    }
}
