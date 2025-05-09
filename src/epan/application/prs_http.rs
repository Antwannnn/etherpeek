use crate::epan::proto_tree::ProtoTree;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct HTTPDissector {

    name: &'static str

}

impl HTTPDissector {
    pub fn new() -> Self {
        HTTPDissector {
            name: "Ethernet IEEE802.3 Frame"
        }
    }
}

impl ProtocolDissector for HTTPDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is
    **/
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult {
        todo!("Implement parser for http")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}