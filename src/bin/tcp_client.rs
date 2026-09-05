use std::io::Write;
use std::net::TcpStream;

fn main()
{
        let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

        let message = b"hello";

        let length: u32 = message.len().try_into().unwrap();

        stream.write_all(&length.to_be_bytes()).unwrap();
        stream.write_all(message).unwrap();
}