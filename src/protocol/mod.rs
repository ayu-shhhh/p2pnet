use std::io::{Read, Write};

pub fn read_message(stream: &mut impl Read) -> Result<Vec<u8>, std::io::Error>
{
        let mut message_length_buffer = [0u8; 4];
        stream.read_exact(&mut message_length_buffer)?;
        let length = usize::try_from(u32::from_be_bytes(message_length_buffer)).unwrap();

        let mut payload = vec!(0u8; length);
        stream.read_exact(&mut payload)?;

        Ok(payload)
}

pub fn write_message(stream: &mut impl Write, message: &[u8]) -> Result<(), std::io::Error>
{
        let length: u32 = message.len().try_into().unwrap();

        stream.write_all(&length.to_be_bytes())?;        
        stream.write_all(message)?;
        
        Ok(())
}
