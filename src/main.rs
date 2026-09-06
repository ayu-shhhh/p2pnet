use std::net::TcpListener;
use p2pnet::protocol;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Server listening!");

    let (mut stream, address) = listener.accept().unwrap();
    println!("Connection from {}", address);

    let payload = protocol::read_message(&mut stream);
    println!("Payload bytes: {:?}", str::from_utf8(&payload).unwrap());
}