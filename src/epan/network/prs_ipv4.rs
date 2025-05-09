use std::collections::HashMap;
use std::net::Ipv4Addr;
use crate::epan::osi::OsiLayer;
use crate::epan::parse_error::ParseError;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::epan::protocol_result::ProtocolDissectResult::{Error, Parsed};
use crate::epan::sub_dissector_caller::SubDissectorCaller;
use crate::io::packet_buffer::PktBuf;

pub struct IPv4Dissector {
    name: &'static str,
}

impl IPv4Dissector {
    pub fn new() -> Self {
        IPv4Dissector {
            name: "IPv4"
        }
    }

    pub fn protocol_description(val: u8) -> &'static str {
        match val {
            1 => "ICMP",
            2 => "IGMP",
            6 => "TCP",
            17 => "UDP",
            58 => "ICMPv6",
            _ => "Unknown",
        }
    }
}

impl SubDissectorCaller for IPv4Dissector {
    fn call_subsequent_dissector(&self, pkt_buf: &PktBuf, proto_tree: &mut ProtoTree, parent_node: &mut ProtoTreeNode) {
        todo!()
    }
}

impl ProtocolDissector for IPv4Dissector {
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        if self.can_dissect(buffer) {
            let version_ihl = *buffer.read_next_u8(0).unwrap();
            let version = (version_ihl >> 4) & 0x0F;
            let ihl = version_ihl & 0x0F;
            let header_len = (ihl * 4) as usize;

            let dscp_ecn = *buffer.read_next_u8(1).unwrap();
            let dscp = (dscp_ecn >> 2) & 0x3F;
            let ecn = dscp_ecn & 0x03;

            let total_length = buffer.read_next_u16(2).unwrap();
            let identification = buffer.read_next_u16(4).unwrap();
            
            let flags_fragment = buffer.read_next_u16(6).unwrap();
            let flags = ((flags_fragment >> 13) & 0x07) as u8;
            let fragment_offset = flags_fragment & 0x1FFF;

            let ttl = *buffer.read_next_u8(8).unwrap();
            let protocol = *buffer.read_next_u8(9).unwrap();
            let header_checksum = buffer.read_next_u16(10).unwrap();

            let source_bytes = buffer.byte_range_checked(12, 4).unwrap();
            let source = Ipv4Addr::new(source_bytes[0], source_bytes[1], source_bytes[2], source_bytes[3]);

            let dest_bytes = buffer.byte_range_checked(16, 4).unwrap();
            let destination = Ipv4Addr::new(dest_bytes[0], dest_bytes[1], dest_bytes[2], dest_bytes[3]);

            let fields = {
                let mut map = HashMap::new();
                map.insert("version".to_string(), version.to_string());
                map.insert("ihl".to_string(), ihl.to_string());
                map.insert("dscp".to_string(), dscp.to_string());
                map.insert("ecn".to_string(), ecn.to_string());
                map.insert("total_length".to_string(), total_length.to_string());
                map.insert("identification".to_string(), format!("0x{:04X}", identification));
                map.insert("flags".to_string(), format!("0x{:X}", flags));
                map.insert("fragment_offset".to_string(), fragment_offset.to_string());
                map.insert("ttl".to_string(), ttl.to_string());
                map.insert("protocol".to_string(), format!("{} ({})", protocol, Self::protocol_description(protocol)));
                map.insert("header_checksum".to_string(), format!("0x{:04X}", header_checksum));
                map.insert("source".to_string(), source.to_string());
                map.insert("destination".to_string(), destination.to_string());
                map
            };

            let mut proto_tree_node = ProtoTreeNode::new(self.name.to_string());
            proto_tree_node.proto_infos = fields;
            prototree.insert_node(proto_tree_node, parent_node);
            
            Parsed
        } else {
            Error(ParseError::InvalidHeader)
        }
    }

    fn can_dissect(&self, buf: &PktBuf) -> bool {
        if buf.length < 20 {
            return false;
        }

        match buf.read_next_u8(0) {
            Some(version_ihl) => {
                let version = (*version_ihl >> 4) & 0x0F;
                version == 4
            }
            None => false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}