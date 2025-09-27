use std::{
    error::Error,
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

use crate::{http::request::HttpRequest, utils::size::format_size};

pub const HEADER_PROXY_DESTINATION: &str = "x-proxy-destination";

// the issue that is we are creating a new Proxy for each request
// maybe have a way to keep only one proxy per TcpConnection
pub struct Proxy {
    pub source: TcpStream,
    pub target: TcpStream,
    pub request: HttpRequest,
}

impl Proxy {
    pub fn new(mut tcp_stream: TcpStream) -> Result<Proxy, Box<dyn Error>> {
        let request = HttpRequest::parse(&mut tcp_stream)?;
        let tcp_url = format!(
            "{}:{}",
            request.destination.hostname, request.destination.port
        );
        let target = TcpStream::connect(tcp_url)?; // 502 status code

        return Ok(Self {
            source: tcp_stream,
            target,
            request,
        });
    }

    pub fn handle_client(&mut self) -> Result<usize, Box<dyn Error>> {
        self.forward_request();
        let (size, response) = self.read_response()?;

        // TODO: use Proxy::send_response()
        self.source.write_all(&response[..size]).unwrap();

        return Ok(size);
    }

    pub fn send_error_response(mut tcp_stream: TcpStream, error: Box<dyn Error>) {
        let err = error.to_string();
        Proxy::send_response(&mut tcp_stream, 400, "Bad Request".to_string(), err);
    }

    fn forward_request(&mut self) {
        if let Err(e) = self.target.write(&self.request.to_bytes()) {
            eprintln!("Error forwarding request: {}", e);
            let err_response = "HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n";
            let _ = self.source.write_all(err_response.as_bytes());
            return;
        }
    }

    // TODO: have a HttpResponse struct
    fn read_response(&mut self) -> Result<(usize, [u8; 4096]), Box<dyn Error>> {
        let mut buffer = [0; 4096];

        let mut size = 0;
        loop {
            match self.target.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => size += n,
                Err(e) => {
                    eprintln!("Error reading response: {}", e);
                    return Err(Box::new(e));
                }
            };
        }
        return Ok((size, buffer));
    }

    // TODO: have a HttpCode struct to easly match status code + description
    // E.g. 404 -> Not found
    fn send_response(
        tcp_stream: &mut TcpStream,
        status: u16,
        description: String,
        content: String,
    ) {
        let body = format!("{}\r\n", content);
        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            description,
            body.len(),
            body
        );
        let _ = tcp_stream.write_all(response.as_bytes());
    }
}
