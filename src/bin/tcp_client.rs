use std::net::TcpStream;
use p2pnet::protocol;

fn main() -> Result<(), std::io::Error>
{
        let mut stream = TcpStream::connect("127.0.0.1:8000")?;

        let message = b"pikachu";
        protocol::write_message(&mut stream, message)?;
        let message = b"pikachu";
        protocol::write_message(&mut stream, message)?;
        let message = b"pikachu";
        protocol::write_message(&mut stream, message)?;
        
        Ok(())
}