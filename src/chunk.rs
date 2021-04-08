use crate::{chunk_type::ChunkType, Error, Result};
use crc::crc32;
use std::{
    convert::TryFrom,
    fmt,
    io::{BufReader, Read},
    str,
};

pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    message_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let data_length = data.len() as u32;
        let message_data = data;
        let crc_data: Vec<u8> = chunk_type
            .bytes()
            .to_vec()
            .iter()
            .chain(message_data.iter())
            .copied()
            .collect();
        let crc = crc32::checksum_ieee(crc_data.as_ref());
        Self {
            data_length,
            chunk_type,
            message_data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.data_length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.message_data.as_ref()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(str::from_utf8(&self.data())
            .expect("Couldn't convert chunk_data to string")
            .to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.data_length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.message_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = Default::default();

        reader.read_exact(&mut buffer)?;
        let data_length = u32::from_be_bytes(buffer);

        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        let mut buffer: [u8; 1] = Default::default();
        let mut message_data = Vec::new();
        for _ in 0..data_length {
            reader.read_exact(&mut buffer)?;
            message_data.append(&mut buffer.to_vec());
        }

        let mut buffer: [u8; 4] = Default::default();
        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        let data_end_index = 8 + data_length as usize;
        if crc32::checksum_ieee(&bytes[4..data_end_index]) != crc {
            Err(Error::from("Invalid CRC"))
        } else {
            Ok(Self {
                data_length,
                chunk_type,
                message_data,
                crc,
            })
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk {{")?;
        write!(f, " Length: {},", self.length())?;
        write!(f, " Type: {},", self.chunk_type())?;
        write!(f, " Data: {} bytes,", self.data().len())?;
        write!(f, " Crc: {}", self.crc())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
