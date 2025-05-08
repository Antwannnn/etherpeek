use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;
use crate::io::protocol::Protocol;

pub struct TCPDissector {
    name: &'static str
}

impl TCPDissector {
    pub fn new() -> Self {
        TCPDissector {
            name: "TCP"
        }
    }
}

impl ProtocolDissector for TCPDissector {
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        todo!("Implement parser for tcp")
    }

    fn name(&self) -> &'static str {
        self.name
    }
}