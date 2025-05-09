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

    pub fn read_next_u8(&self, offset: usize) -> Option<&u8> {
        self.buf.get(offset)
    }

    pub fn read_next_u16(&self, offset: usize) -> Option<u16> {
        if offset + 2 > self.packet.buf.len() {
            return None;
        }

        let bytes = &self.packet.buf[offset..offset + 2];
        Some(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub fn read_next_u32(&self, offset: usize) -> Option<u32> {
        if offset + 2 > self.packet.buf.len() {
            return None;
        }

        let bytes = &self.packet.buf[offset..offset + 4];
        Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) // or from_be_bytes depending on endianness
    }

    pub fn read_next_u64(&self, offset: usize) -> Option<u64> {
        if offset + 2 > self.packet.buf.len() {
            return None;
        }

        let bytes = &self.packet.buf[offset..offset + 8];
        Some(u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) // or from_be_bytes depending on endianness
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