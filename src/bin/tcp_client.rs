use std::io;
use std::net::TcpStream;
use std::thread;

use p2pnet::protocol;

fn main() -> Result< (), p2pnet::error::ProtocolError >
{
        let mut stream = TcpStream::connect("127.0.0.1:8000")?;
        let mut receive_stream = stream.try_clone()?;

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

        // let message = b"oioi";
        // protocol::write_message(&mut stream, message)?;
        // let message = b"oioi";
        // protocol::write_message(&mut stream, message)?;
        // let message = b"oioi";
        // protocol::write_message(&mut stream, message)?;
        
        Ok(())
}