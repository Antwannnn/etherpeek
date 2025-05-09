use std::fmt::{Display, Formatter};
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::io::packet_buffer::PktBuf;

pub struct ProtoTree {
    pub nodes: Vec<ProtoTreeNode>,
}

impl ProtoTree {
    pub fn new() -> Self {
        ProtoTree {
            nodes: Vec::new(),
        }
    }
}

impl Display for ProtoTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        Ok(())
    }
}