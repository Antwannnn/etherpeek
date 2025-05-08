use std::collections::HashMap;
use std::io::Error;
use crate::epan::osi::OsiLayer;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;
use crate::io::protocol::Protocol;

pub struct HTMLDissector {

    name: &'static str

}

impl HTMLDissector {
    pub fn new() -> Self {
        HTMLDissector {
            name: "Ethernet IEEE802.3"
        }
    }
}

impl ProtocolDissector for HTMLDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is 
    **/
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        todo!("Implement parser for html")
    }

    fn name(&self) -> &'static str {
        self.name
    }
}