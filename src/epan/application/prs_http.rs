use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;
use crate::io::protocol::Protocol;

pub struct HTTPDissector {

    name: &'static str

}

impl HTTPDissector {
    pub fn new() -> Self {
        HTTPDissector {
            name: "Ethernet IEEE802.3 Frame (MAC)"
        }
    }
}

impl ProtocolDissector for HTTPDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is
    **/
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        todo!("Implement parser for http")
    }

    fn name(&self) -> &'static str {
        self.name
    }
}