use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct IEEE8023FrameDissector {

    name: &'static str

}

impl IEEE8023FrameDissector {
    pub fn new() -> Self {
        IEEE8023FrameDissector {
            name: "Ethernet IEEE802.3 Frame (MAC)"
        }
    }
}

impl ProtocolDissector for IEEE8023FrameDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is
    **/
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        todo!("Implement parser for ieee802.3 frame")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}