use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

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
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult {
        todo!("Not implemented yet")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}