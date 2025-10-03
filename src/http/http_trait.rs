use std::{collections::HashMap, error::Error, io::BufRead};

pub trait Http {
    fn get_first_line(&self) -> String;
    fn get_headers(&self) -> HashMap<String, String>;
    fn get_body(&self) -> Option<Vec<u8>>;

    fn parse_first_line<R: BufRead>(
        reader: &mut R,
    ) -> Result<(String, String, String), Box<dyn Error>> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        return Ok((
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        ));
    }

    fn parse_headers<R: BufRead>(
        reader: &mut R,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let mut headers = HashMap::new();

        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            line = line.trim().to_string();

            if line.is_empty() {
                break;
            }

            if let Some(colon_pos) = line.find(":") {
                let key = line[..colon_pos].trim().to_lowercase();
                let value = line[colon_pos + 1..].trim().to_string();

                headers.insert(key, value);
            }
        }
        return Ok(headers);
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let first_line = format!("{}\r\n", self.get_first_line());
        result.extend_from_slice(first_line.as_bytes());

        let mut headers = self.get_headers();
        let body = self.get_body();
        if !headers.contains_key("content-length") && body.is_some() {
            headers.insert(
                "Content-Length".to_string(),
                body.clone().unwrap().len().to_string(),
            );
        }

        for (key, value) in headers {
            // remove transfer-encoding because we send parsed body and not chunked body
            if key != "transfer-encoding" {
                let header_line = format!("{}: {}\r\n", key, value);
                result.extend_from_slice(header_line.as_bytes());
            }
        }

        result.extend_from_slice(b"\r\n"); // End of headers

        if body.is_some() {
            result.extend_from_slice(&body.unwrap());
        }

        return result;
    }
}
