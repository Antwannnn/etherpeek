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
    pub fn insert_node<'a>(
        &'a mut self,
        node: ProtoTreeNode,
        parent: Option<&'a mut ProtoTreeNode>
    ) -> &'a mut ProtoTreeNode {
        if let Some(parent_node) = parent {
            parent_node.children.push(node);
            parent_node.children.last_mut().unwrap()
        } else {
            self.nodes.push(node);
            self.nodes.last_mut().unwrap()
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