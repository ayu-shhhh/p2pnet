use std::io;
use std::net::TcpListener;
use std::thread;

use p2pnet::protocol;

fn main() -> Result<(), p2pnet::error::ProtocolError>
{

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server listening!");

    let (mut stream, address) = listener.accept()?;
    let mut receive_stream = stream.try_clone()?;
    println!("Connection from {}", address);


    thread::spawn(move || {
        loop
        {
            match protocol::read_message(&mut receive_stream)
            {
                Ok(Some(payload)) => {
                    println!("Peer: {:?}", str::from_utf8(&payload).unwrap());
                }

                Ok(None) => {
                    println!("Peer disconnected!");
                    break;
                }
                
                Err(error) => {
                    eprintln!("Receive error: {:?}", error);
                    break;
                }
            }
        }
    });

    loop
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        protocol::write_message(&mut stream, input.trim_end().as_bytes())?;
    }
}