use std::ops::Deref;
use super::peek::peek::Peek;

pub struct PktBuf{

    pub length: u16,

    pub packet: Peek

}

impl PktBuf {
    pub fn new(pkt: Peek) -> Self {
        PktBuf{
            length: pkt.length,
            packet: pkt
        }
    }

    pub fn byte_range(&self, start: usize, len: usize) -> Option<&[u8]> {
        self.buf.get(start..start + len)
    }

    pub fn byte_range_checked(&self, start: usize, len: usize) -> Result<&[u8], String> {
        self.buf
            .get(start..start + len)
            .ok_or_else(|| format!("Requested byte range {}..{} is out of bounds", start, start + len))
    }

    pub fn rep_len_remaining(&self, position: usize) -> Option<u16> {
        if position > self.buf.len(){
            return None;
        }
        Some((self.buf.len() - position) as u16)
    }
    
}

impl Deref for PktBuf {
    type Target = Peek;

    fn deref(&self) -> &Self::Target {
        &self.packet
    }
}