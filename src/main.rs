mod chunk_type;
mod chunk;
mod png;
use std::{fs, str::FromStr};

use chunk_type::ChunkType;
use chunk::Chunk;
use png::Png;

fn main() {
    let input = fs::read("img.png").unwrap();
    let chunk_type = ChunkType::from_str("ruSt").unwrap();
    let data = "this is a secret message".as_bytes().to_vec();
    let chunk = Chunk::new(chunk_type, data);
    let png = Png::from_chunks(vec![chunk]);
    let result = [input, png.as_bytes()].concat();
    fs::write("img.png", result).unwrap();
}