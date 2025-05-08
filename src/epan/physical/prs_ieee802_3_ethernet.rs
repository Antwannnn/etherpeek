use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;
use crate::io::protocol::Protocol;

pub struct IEEE802_3Dissector {

    name: &'static str

}

impl IEEE802_3Dissector {
    pub fn new() -> Self {
        IEEE802_3Dissector {
            name: "Ethernet IEEE802.3"
        }
    }
}

impl ProtocolDissector for IEEE802_3Dissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is 
    **/
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        todo!("Implement parser for ieee802.3@")
    }

    fn name(&self) -> &'static str {
        self.name
    }
}