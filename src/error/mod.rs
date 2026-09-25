#[derive(Debug)]
pub enum ProtocolError
{
    Io(std::io::Error),
    MessageTooLarge,
}

impl From<std::io::Error> for ProtocolError
{
    fn from(error: std::io::Error) -> Self
    {
        ProtocolError::Io(error)
    }
}