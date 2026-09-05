use std::net::TcpListener;
use std::io::{Read};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Server listening!");

    let (mut stream, address) = listener.accept().unwrap();
    println!("Connection from {}", address);

    let mut message_length_buffer = [0u8; 4];
    stream.read_exact(&mut message_length_buffer).unwrap();
    println!("Read bytes : {:?}", message_length_buffer);

    let message_length = u32::from_be_bytes(message_length_buffer);
    println!("Message length: {}", message_length);

    let mut payload = vec![0u8; message_length.try_into().unwrap()];
    stream.read_exact(&mut payload).unwrap();
    println!("Payload bytes: {:?}", str::from_utf8(&payload).unwrap());
}