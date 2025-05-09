use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct SSHDissector {

    name: &'static str

}

impl SSHDissector {
    pub fn new() -> Self {
        SSHDissector {
            name: "SSH"
        }
    }
}

impl ProtocolDissector for SSHDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is
    **/
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult {
        todo!("Implement parser for ssh")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}