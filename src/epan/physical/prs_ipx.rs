use crate::epan::parse_error::ParseError::UnsupportedProtocol;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::epan::protocol_result::ProtocolDissectResult::Error;
use crate::io::packet_buffer::PktBuf;

pub struct IPXDissector {
    name: &'static str
}

impl IPXDissector {
    pub fn new() -> Self {
        IPXDissector {
            name: "IPX"
        }
    }
}

impl ProtocolDissector for IPXDissector {
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        Error(UnsupportedProtocol)
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str{
        self.name
    }
}
