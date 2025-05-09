use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct ProtoTreeNode {
    pub name: String,
    
    pub children: Vec<ProtoTreeNode>,
    
    pub proto_infos: HashMap<String, String>
}

impl ProtoTreeNode {
    pub fn new(name: String) -> Self {
        ProtoTreeNode{
            name,
            children: Vec::new(),
            proto_infos: HashMap::new()
        }
    }

    fn fmt_with_indent(&self, f: &mut Formatter<'_>, prefix: &str, is_last: bool) -> std::fmt::Result {
        let branch = if is_last { "└── " } else { "├── " };
        writeln!(f, "{}{}{}", prefix, branch, self.name)?;

        // Print proto_infos with appropriate indentation
        if !self.proto_infos.is_empty() {
            let info_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
            writeln!(f, "{}[Fields]", info_prefix)?;
            for (key, value) in &self.proto_infos {
                writeln!(f, "{}  • {}: {}", info_prefix, key, value)?;
            }
        }

        let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };

        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i == self.children.len() - 1;
            child.fmt_with_indent(f, &new_prefix, is_last_child)?;
        }

        Ok(())
    }
}

impl Display for ProtoTreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_indent(f, "", true)
    }
}