use std::{io::Write, net::TcpStream, time::Instant};

use crate::{
    http::{request::HttpRequest, url::URL},
    utils,
};

const HEADER_PROXY_DESTINATION: &str = "x-proxy-destination";

fn get_destination(req: &HttpRequest) -> Result<URL, String> {
    if let Some(destination) = req.headers.get(HEADER_PROXY_DESTINATION) {
        return Ok(URL::new(&destination)?);
    };

    let config = utils::config::get_config();
    if let Some(ref destination) = config.default_destination {
        return Ok(URL::new(&destination)?);
    }

    return Err("No destination found".into());
}

pub fn handle_client(mut source_stream: TcpStream) {
    let mut req = match HttpRequest::parse(&mut source_stream) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error parsing request: {}", e);
            return;
        }
    };

    let url_destination = match get_destination(&req) {
        Ok(dest) => dest,
        Err(err) => {
            let response = format!(
                "HTTP/1.1 400 Bad Request\r\n\
                 Content-Length: {}\r\n\
                 \r\n\
                 {}",
                err.len(),
                err
            );
            let _ = source_stream.write_all(response.as_bytes());
            return;
        }
    };

    let mut target_stream: TcpStream = match TcpStream::connect(format!(
        "{}:{}",
        url_destination.hostname, url_destination.port
    )) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!(
                "Error connecting to {}\r\n- {}",
                url_destination.to_string(),
                e
            );
            let response = "HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n";
            let _ = source_stream.write_all(response.as_bytes());
            return;
        }
    };

    let date = chrono::Utc::now().format("%H:%M:%S");
    let start_time = Instant::now();
    // stuff that forward the request
    // request received, get duration
    let duration = start_time.elapsed();
    let status = "200";

    // Remove proxy destination before forwarding the request
    req.headers.remove(HEADER_PROXY_DESTINATION);

    println!(
        "[{}] {} {} -> {} ({:.2}ms) [{}]",
        date,
        req.method,
        req.path,
        url_destination.to_string(),
        duration.as_secs_f64() * 1000.0,
        status
    )
}
