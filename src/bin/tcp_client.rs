// use std::io::Write;
use std::net::TcpStream;
use p2pnet::protocol;

fn main()
{
        let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

        let message = b"pikachu";
        protocol::write_message(&mut stream, message);
}