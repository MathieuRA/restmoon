mod http;
mod utils;

use std::{net::TcpListener, thread};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::print::initial_log();

    let config = utils::config::get_config();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.listen_port))?;

    for stream in listener.incoming() {
        match stream {
            Ok(client_stream) => {
                thread::spawn(move || {
                    utils::proxy::handle_client(client_stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    return Ok(());
}
