use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType{
    pub bytes: [u8;4]
}

impl ChunkType{
    pub fn is_bit_zero(byte: u8, i:u8)->bool{
        let mask = 1 << i; // <--- major doubts here
        byte & mask == 0
    }
    pub fn is_valid_byte(byte:u8)->bool{
        (65<=byte && byte<=90)||(97<=byte&&byte<=122)
    }
    pub fn bytes(&self)->[u8;4]{
        self.bytes
    }
    pub fn is_valid(&self)->bool{
        Self::is_bit_zero(self.bytes[2], 5)
    }
    pub fn is_critical(&self)->bool{
        Self::is_bit_zero(self.bytes[0], 5)
    }
    pub fn is_public(&self)->bool{
        Self::is_bit_zero(self.bytes[1], 5)
    }
    pub fn is_reserved_bit_valid(&self)->bool{
        Self::is_bit_zero(self.bytes[2], 5)
    }
    pub fn is_safe_to_copy(&self)->bool{
        !Self::is_bit_zero(self.bytes[3], 5)
    }
}

impl fmt::Display for ChunkType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.bytes{
            write!(f, "{}", char::from(*b))?;
        } Ok(()) 
    } 
}

impl FromStr for ChunkType{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len()!=4{
            return Err("Not valid length");
        }

        let mut bytes = [0;4];

        for (i, byte) in s.as_bytes().iter().enumerate(){
            if Self::is_valid_byte(*byte){
                bytes[i] = *byte;
            } else {
                return Err("Bytes are not valid");
            }
        }
        return Ok(ChunkType { bytes });
    }
}

impl TryFrom<[u8;4]> for ChunkType{

    type Error = &'static str;

    fn try_from(bytes: [u8;4]) -> Result<Self, Self::Error> {
        for byte in bytes.iter(){
            if !Self::is_valid_byte(*byte){
                return Err("Not valid bytes");
            }
        }
        return Ok(ChunkType{bytes});
    }
}

//bLOb  <-- 32 bit chunk type code represented in text form
//||||
//|||+- Safe-to-copy bit is 1 (lowercase letter; bit 5 is 1)
//||+-- Reserved bit is 0     (uppercase letter; bit 5 is 0)
//|+--- Private bit is 0      (uppercase letter; bit 5 is 0)
//+---- Ancillary bit is 1    (lowercase letter; bit 5 is 1)