mod http;
mod utils;

use std::{net::TcpListener, thread, time::Instant};

use crate::utils::{print::final_log, proxy::Proxy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::print::initial_log();

    let config = utils::config::get_config();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.listen_port))?;

    for stream in listener.incoming() {
        match stream {
            Ok(client_stream) => {
                thread::spawn(move || {
                    // TODO: better handling of error
                    // create a new HttpErrorStruct
                    // so i can throw it and catch it easly in send_error_response
                    // in order to use dynamic status code / message
                    let mut proxy = match Proxy::new(
                        client_stream
                            .try_clone()
                            .expect("Unable to copy the TCP stream"),
                    ) {
                        Ok(proxy) => proxy,
                        Err(error) => {
                            Proxy::send_error_response(client_stream, error);
                            return;
                        }
                    };

                    let date = chrono::Utc::now().format("%H:%M:%S");
                    let start_time = Instant::now();

                    let size = match proxy.handle_client() {
                        Ok(size) => size,
                        Err(error) => {
                            Proxy::send_error_response(client_stream, error);
                            0
                        }
                    };

                    let duration = start_time.elapsed();

                    final_log(&proxy, date, duration, size);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    return Ok(());
}
