use std::collections::HashMap;
use std::io::Error;
use crate::epan::osi::OsiLayer;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct TLSDissector {

    name: &'static str

}

impl TLSDissector {
    pub fn new() -> Self {
        TLSDissector {
            name: "Ethernet IEEE802.3"
        }
    }
}

impl ProtocolDissector for TLSDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is 
    **/
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult {
        todo!("Implement parser for tls")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}