use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct DNSDissector {name: &'static str }

impl DNSDissector {
    pub fn new() -> Self {
    DNSDissector {
        name: "DNS"
    }
    }
}

impl ProtocolDissector for DNSDissector {
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        todo!("Implement parser for dns")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str{self.name}

}
