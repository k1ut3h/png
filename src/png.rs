use crate::chunk::Chunk;

pub struct Png{
    chunks: Vec<Chunk>
}

impl Png{
    const STANDARD_HEADER:[u8;8] = [137,80,78,71,13,10,26,10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png{
        Png{chunks}
    } 
    pub fn append_chunk(&mut self, chunk: Chunk){
        self.chunks.push(chunk);
    }
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk,&'static str>{
        for (i, chunk) in self.chunks.iter().enumerate(){
            if chunk.chunk_type().bytes()==chunk_type.as_bytes(){
                return Ok(self.chunks.remove(i));
            }
        }
        return Err("Unable to remove chunk");
    }
    pub fn header(&self) -> &[u8; 8]{
        &Self::STANDARD_HEADER
    }
    pub fn chunks(&self) -> &[Chunk]{
        &self.chunks
    }
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk>{
        for chunk in self.chunks(){
            if chunk.chunk_type().bytes()==chunk_type.as_bytes(){
                return Some(chunk);
            }
        }
        return None;
    }
    pub fn as_bytes(&self) -> Vec<u8>{
        let chunk_iterators: Vec<u8> = self.chunks.iter().map(|c| c.as_bytes()).flatten().collect();
        self.header()
            .iter()
            .chain(chunk_iterators.iter())
            .copied()
            .collect()
    }
}