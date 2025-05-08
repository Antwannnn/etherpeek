use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;
use crate::io::protocol::Protocol;

pub struct IPV4Dissector {

    name: &'static str

}

impl IPV4Dissector {
    pub fn new() -> Self {
        IPV4Dissector {
            name: "IPV4"
        }
    }
}

impl ProtocolDissector for IPV4Dissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is
    **/
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        ProtocolDissectResult::Parsed(Protocol::new(
            self.name.parse().unwrap(),
            OsiLayer::Network,
            Some(0x03),
            HashMap::new(),
            buffer.buf.clone(),
            buffer.buf.len()
        ))
    }

    fn name(&self) -> &'static str {
        self.name
    }
}