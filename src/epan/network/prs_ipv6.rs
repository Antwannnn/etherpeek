use std::collections::HashMap;
use std::net::Ipv6Addr;
use crate::epan::osi::OsiLayer;
use crate::epan::parse_error::ParseError;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::epan::protocol_result::ProtocolDissectResult::{Error, Parsed};
use crate::epan::sub_dissector_caller::SubDissectorCaller;
use crate::io::packet_buffer::PktBuf;

pub struct IPv6Dissector {
    name: &'static str,
}

impl IPv6Dissector {
    pub fn new() -> Self {
        IPv6Dissector {
            name: "IPv6"
        }
    }

    pub fn next_header_description(val: u8) -> &'static str {
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

impl SubDissectorCaller for IPv6Dissector {
    fn call_subsequent_dissector(&self, pkt_buf: &PktBuf, proto_tree: &mut ProtoTree, parent_node: &mut ProtoTreeNode) {
        todo!()
    }
}

impl ProtocolDissector for IPv6Dissector {
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        if self.can_dissect(buffer) {
            let version_tc_flow = buffer.read_next_u32(0).unwrap();
            let version = ((version_tc_flow >> 28) & 0x0F) as u8;
            let traffic_class = ((version_tc_flow >> 20) & 0xFF) as u8;
            let flow_label = version_tc_flow & 0xFFFFF;

            let payload_length = buffer.read_next_u16(4).unwrap();
            let next_header = *buffer.read_next_u8(6).unwrap();
            let hop_limit = *buffer.read_next_u8(7).unwrap();

            let source_bytes = buffer.byte_range_checked(8, 16).unwrap();
            let source = Ipv6Addr::from([
                source_bytes[0], source_bytes[1], source_bytes[2], source_bytes[3],
                source_bytes[4], source_bytes[5], source_bytes[6], source_bytes[7],
                source_bytes[8], source_bytes[9], source_bytes[10], source_bytes[11],
                source_bytes[12], source_bytes[13], source_bytes[14], source_bytes[15],
            ]);

            let dest_bytes = buffer.byte_range_checked(24, 16).unwrap();
            let destination = Ipv6Addr::from([
                dest_bytes[0], dest_bytes[1], dest_bytes[2], dest_bytes[3],
                dest_bytes[4], dest_bytes[5], dest_bytes[6], dest_bytes[7],
                dest_bytes[8], dest_bytes[9], dest_bytes[10], dest_bytes[11],
                dest_bytes[12], dest_bytes[13], dest_bytes[14], dest_bytes[15],
            ]);

            let fields = {
                let mut map = HashMap::new();
                map.insert("version".to_string(), version.to_string());
                map.insert("traffic_class".to_string(), traffic_class.to_string());
                map.insert("flow_label".to_string(), format!("0x{:05X}", flow_label));
                map.insert("payload_length".to_string(), payload_length.to_string());
                map.insert("next_header".to_string(), format!("{} ({})", next_header, Self::next_header_description(next_header)));
                map.insert("hop_limit".to_string(), hop_limit.to_string());
                map.insert("source".to_string(), source.to_string());
                map.insert("destination".to_string(), destination.to_string());
                map
            };

            let mut proto_tree_node = ProtoTreeNode::new(self.name.to_string());
            proto_tree_node.proto_infos = fields;
            prototree.nodes.push(proto_tree_node);
            
            Parsed
        } else {
            Error(ParseError::InvalidHeader)
        }
    }

    fn can_dissect(&self, buf: &PktBuf) -> bool {
        if buf.length < 40 {
            return false;
        }

        match buf.read_next_u8(0) {
            Some(version_tc) => {
                let version = (*version_tc >> 4) & 0x0F;
                version == 6
            }
            None => false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}