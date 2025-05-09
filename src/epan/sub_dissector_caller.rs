use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub trait SubDissectorCaller{
    fn call_subsequent_dissector(&self, pkt_buf: &PktBuf, proto_tree: &mut ProtoTree, parent_node: &mut ProtoTreeNode);
}