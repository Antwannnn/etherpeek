use crate::io::packet_buffer::PktBuf;

pub trait SubDissectorCaller{
    fn call_subsequent_dissector(pkt_buf: &PktBuf);
}