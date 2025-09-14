use std::net::TcpStream;

pub fn handle_client(tcp_stream: TcpStream) {
    println!("Got a TCP connexion! {:?}", tcp_stream)
}
