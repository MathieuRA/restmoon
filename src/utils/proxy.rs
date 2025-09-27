use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

use crate::{http::request::HttpRequest, utils::size::format_size};

pub const HEADER_PROXY_DESTINATION: &str = "x-proxy-destination";

pub fn handle_client(mut source_stream: TcpStream) {
    let req = match HttpRequest::parse(&mut source_stream) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error parsing request: {}", e);
            let error = e.to_string();
            let err_response = format!(
                "HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
                error.len(),
                error
            );
            let _ = source_stream.write_all(err_response.as_bytes());
            return;
        }
    };

    let mut target_stream: TcpStream = match TcpStream::connect(format!(
        "{}:{}",
        req.destination.hostname, req.destination.port
    )) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!(
                "Error connecting to {}\r\n- {}",
                req.destination.to_string(),
                e
            );
            let err_response = "HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n";
            let _ = source_stream.write_all(err_response.as_bytes());
            return;
        }
    };

    let date = chrono::Utc::now().format("%H:%M:%S");
    let start_time = Instant::now();
    if let Err(e) = target_stream.write_all(&req.to_bytes()) {
        eprintln!("Error forwarding request: {}", e);
        let err_response = "HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n";
        let _ = source_stream.write_all(err_response.as_bytes());
        return;
    };

    let mut response_size = 0;
    let mut buffer = [0; 4096];

    loop {
        match target_stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                response_size += n;
                source_stream.write_all(&buffer[..n]).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading response: {}", e);
                break;
            }
        }
    }
    let duration = start_time.elapsed();

    println!(
        "[{}] {} {} -> {} ({:.2}ms) [Response: {}]",
        date,
        req.method,
        req.path,
        req.destination.to_string(),
        duration.as_secs_f64() * 1000.0,
        format_size(response_size)
    )
}
