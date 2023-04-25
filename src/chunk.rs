use crate::chunk_type::ChunkType;
use std::{string::{self, FromUtf8Error}, fmt};
use crc32fast;

pub struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl Chunk{
    pub fn new(chunk_type: ChunkType, data: Vec<u8>)->Chunk{
        let length = data.len().try_into().unwrap();
        let crc = crc32fast::hash(&[&chunk_type.bytes(), &data[..]].concat()); 
        Chunk{length, chunk_type, data, crc}
    }
    pub fn length(&self)->u32{
        self.length
    }
    pub fn chunk_type(&self)->&ChunkType{
        &self.chunk_type
    }
    pub fn data(&self)->&[u8]{
        self.data.as_slice()
    }
    pub fn as_bytes(&self) -> Vec<u8>{
        self.data.clone()
    }
    fn data_as_string(&self) -> Result<String, FromUtf8Error>{
        string::String::from_utf8(self.data.clone())
    }
    fn crc(&self)->u32{
        self.crc
    }
}

impl fmt::Display for Chunk{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in &self.data{
            write!(f, "{}", d)?;
        }
        Ok(())
    }
}