use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct UDPDissector {
    name: &'static str
}

impl UDPDissector {
    pub fn new() -> Self {
        UDPDissector {
            name: "UDP"
        }
    }
}

impl ProtocolDissector for UDPDissector {
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        todo!("Implement parser for udp")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}