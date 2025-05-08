use std::fmt::{format, Display};
use pcap::Packet;
use crate::epan::parse_error::ParseError;
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
    
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, ParseError> {
        use std::io::Read;

        let mut timestamp_buf = [0u8; 8];
        let mut length_buf = [0u8; 2];
        let mut format_buf = [0u8; 1];
        let mut id_buf = [0u8; 2];

        bytes.read_exact(&mut timestamp_buf).ok().expect("Failed to read peek header field -> timestamp");
        bytes.read_exact(&mut length_buf).ok().expect("Failed to read peek header field -> length");
        bytes.read_exact(&mut format_buf).ok().expect("Failed to read peek header field -> format");
        bytes.read_exact(&mut id_buf).ok().expect("Failed to read peek header field -> id_buf");

        let length = u16::from_le_bytes(length_buf);
        let mut buf = vec![0u8; length as usize];
        bytes.read_exact(&mut buf).ok().expect("Failed to read peek buffer");

        Ok(Self {
            timestamp: u64::from_le_bytes(timestamp_buf),
            length,
            format: DataFormat::from_u8(format_buf[0]).unwrap(),
            packet_id: u16::from_le_bytes(id_buf),
            buf,
        })
    }
}

impl Display for Peek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timestamp = &self.timestamp;
        let length = &self.length;
        let format = &self.format.as_str();
        let packet_id = &self.packet_id;
        let str = format!("------ Packet: {packet_id} ------\n\
                Timestamp: {timestamp} \n\
                Length: {length} \n\
                Format: {format} \n\
                ").to_string();
        write!(f, "{}", str)
    }   
}
