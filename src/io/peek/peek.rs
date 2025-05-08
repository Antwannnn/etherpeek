use pcap::Packet;
use crate::io::data_format::DataFormat;

pub struct Peek{
    pub timestamp: u64,

    pub length: u16,

    pub format: DataFormat,

    pub packet_id: u16,

    pub buf: Vec<u8>,
}

impl Peek {
    pub fn format_packet(packet: Packet, packet_id: u16, format: DataFormat) -> Self {
        let timestamp = packet.header.ts.tv_sec as u64 * 1_000_000
            + packet.header.ts.tv_usec as u64;
        let length = packet.header.len as u16;
        let buf = packet.data.to_vec();

        Self {
            timestamp,
            length,
            format,
            packet_id,
            buf,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(13 + self.buf.len());
        out.extend(&self.timestamp.to_le_bytes());
        out.extend(&self.length.to_le_bytes());
        out.push(self.format.to_u8()); // 👍 write format as u8
        out.extend(&self.packet_id.to_le_bytes());
        out.extend(&self.buf);
        out
    }
    
    pub fn from_bytes(mut bytes: &[u8]) -> Option<Self> {
        use std::io::Read;

        let mut timestamp_buf = [0u8; 8];
        let mut length_buf = [0u8; 2];
        let mut format_buf = [0u8; 1];
        let mut id_buf = [0u8; 2];

        bytes.read_exact(&mut timestamp_buf).ok()?;
        bytes.read_exact(&mut length_buf).ok()?;
        bytes.read_exact(&mut format_buf).ok()?;
        bytes.read_exact(&mut id_buf).ok()?;

        let length = u16::from_le_bytes(length_buf);
        let mut buf = vec![0u8; length as usize];
        bytes.read_exact(&mut buf).ok()?;

        Some(Self {
            timestamp: u64::from_le_bytes(timestamp_buf),
            length,
            format: DataFormat::from_u8(format_buf[0])?,
            packet_id: u16::from_le_bytes(id_buf),
            buf,
        })
    }
}
