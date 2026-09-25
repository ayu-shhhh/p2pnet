use std::env;

fn main() -> Result<(), p2pnet::error::ProtocolError>
{
    let mut args = env::args();

    args.next();

    match args.next()
    {
        Some(option) if option == "listen" => {
            p2pnet::network::listen()
        }
        
        Some(option) if option == "connect" => {
            p2pnet::network::connect()
        }
        
        _ => {
            eprintln!("Usage: ");
            eprintln!(" p2pnet listen");
            eprintln!(" p2pnet connect");

            Ok(())
        }
    }
}