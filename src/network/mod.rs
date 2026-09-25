use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::error::ProtocolError;
use crate::protocol;

pub fn listen() -> Result<(), ProtocolError>
{
        let listener = TcpListener::bind("127.0.0.1:8000")?;
        println!("Server Listening!");

        let (mut send_stream, address) = listener.accept()?;
        println!("Connection from {}", address);

        let mut receive_stream = send_stream.try_clone()?;

        thread::spawn(move || {
                loop{
                        match protocol::read_message(&mut receive_stream)
                        {
                                Ok(Some(payload)) => {
                                        println!("Peer: {:?}", str::from_utf8(&payload).unwrap());
                                }

                                Ok(None) => {
                                        // println!("Connection closed by Peer!");
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

                if let Err(_error) = protocol::write_message(&mut send_stream, input.trim_end().as_bytes())
                {
                        eprintln!("Failed to send message. Connection closed by peer.");
                        break Ok(());
                }
        }
}

pub fn connect() -> Result< (), ProtocolError>
{
        let mut send_stream = TcpStream::connect("127.0.0.1:8000")?;
        let mut receive_stream = send_stream.try_clone()?;

        thread::spawn(move || {
                loop
                {
                        match protocol::read_message(&mut receive_stream)
                        {
                                Ok(Some(payload)) => {
                                        println!("Peer: {:?}", str::from_utf8(&payload).unwrap());
                                }

                                Ok(None) => {
                                        // println!("Connection closed by Peer!");
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

                if let Err(_error) = protocol::write_message(&mut send_stream, input.trim_end().as_bytes())
                {
                        eprintln!("Failed to send message. Connection closed by peer.");
                        break Ok(());
                }
        }
} 