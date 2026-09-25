use std::env;

fn main() -> Result<(), p2pnet::error::ProtocolError>
{
    let mut args = env::args();

    args.next();
    let mode = args.next();
    let address = match args.next()
    {
        Some(address) => address,
        None => {
            eprintln!("Usage:");
            eprintln!("  p2pnet listen <address:port>");
            eprintln!("  p2pnet connect <address:port>");

            return Ok(());
        }
    };
    let address = address.as_str();
    

    match mode
    {
        Some(option) if option == "listen" => {
            p2pnet::network::listen(address)
        }
        
        Some(option) if option == "connect" => {
            p2pnet::network::connect(address)
        }
        
        _ => {
            eprintln!("Usage: ");
            eprintln!("  p2pnet listen <address:port>");
            eprintln!("  p2pnet connect <address:port>");

            Ok(())
        }
    }
}