use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct ARPDissector {
    name: &'static str
}

impl ARPDissector {
    pub fn new() -> Self {
        ARPDissector {
            name: "ARP"}
    }
}

impl ProtocolDissector for ARPDissector {
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        todo!("Implement parser for arp")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str{self.name}

}
