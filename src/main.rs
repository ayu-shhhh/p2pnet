use std::net::TcpListener;
use p2pnet::protocol;

fn main() -> Result<(), std::io::Error>
{

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server listening!");

    let (mut stream, address) = listener.accept()?;
    println!("Connection from {}", address);

    for _ in 0..3
    {
        let payload = protocol::read_message(&mut stream)?;
        println!("Payload bytes: {:?}", str::from_utf8(&payload).unwrap());
    }

    

    Ok(())
}