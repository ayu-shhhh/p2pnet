use std::net::TcpListener;
use p2pnet::protocol;

// use std::thread;
// use std::time::Duration;

fn main() -> Result<(), std::io::Error>
{

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server listening!");

    let (mut stream, address) = listener.accept()?;
    println!("Connection from {}", address);

    loop
    {
        // let payload;
        match protocol::read_message(&mut stream)?
        {
            Some(payload) => {
                println!("Payload bytes: {:?}", str::from_utf8(&payload).unwrap());
            }
            None => {
                println!("Peer disconnected.");
                break;
            }
        }
        // let duration = Duration::from_secs(2);
        // thread::sleep(duration);
    }

    

    Ok(())
}