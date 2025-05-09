use std::arch::aarch64::vuqadd_s8;
use std::collections::HashMap;
use crate::epan::network::prs_arp::ARPDissector;
use crate::epan::network::prs_ipv4::IPv4Dissector;
use crate::epan::network::prs_ipv6::IPv6Dissector;
use crate::epan::parse_error::ParseError;
use crate::epan::physical::prs_ieee8023_ethernet::EtherType::{EthernetII, IEEE8023};
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::epan::protocol_result::ProtocolDissectResult::{Error, NeedsMoreData, Parsed};
use crate::epan::sub_dissector_caller::SubDissectorCaller;
use crate::io::packet_buffer::PktBuf;


enum EtherType {
    IEEE8023,
    EthernetII,
}

impl EtherType {
    pub fn from_value(val: u16) -> Self {
        if val >= 1500 {
            EthernetII
        } else {
            IEEE8023
        }
    }

    pub fn name(&self) -> String {
        match self {
            EthernetII => "Ethernet II".to_string(),
            IEEE8023 => "IEEE802.3".to_string()
        }
    }
}


pub struct Ieee8023EthernetDissector {

    name: &'static str,
    ether_type: EtherType
}

impl Ieee8023EthernetDissector {
    pub const DEFAULT_ETHER_TYPE: u16 = 0x0800;

    pub fn new() -> Self {
        Ieee8023EthernetDissector {
            name: "Ethernet II",
            ether_type: EtherType::from_value(Self::DEFAULT_ETHER_TYPE)
        }
    }

    pub fn ether_type_dissector(val: u16) -> Option<Box<dyn ProtocolDissector>> {
        match val {
            0x0800 => Some(Box::new(IPv4Dissector::new())),
            0x0806 => Some(Box::new(ARPDissector::new())),
            0x86DD => Some(Box::new(IPv6Dissector::new())),
            _ => None,
        }
    }
}

impl SubDissectorCaller for Ieee8023EthernetDissector {
    fn call_subsequent_dissector(&self, pkt_buf: &PktBuf, proto_tree: &mut ProtoTree, parent_node: &mut ProtoTreeNode) {

        let eth_type: u16 = pkt_buf.read_next_u16(12).unwrap();

        if let Some(mut dissector) = Self::ether_type_dissector(eth_type) {
            let sub_buf = pkt_buf.sub_buf(14).unwrap();
            dissector.protocol_dissector(&sub_buf, proto_tree, Some(parent_node));
        } else {
            parent_node.children.push(ProtoTreeNode::new("Unknown data".to_string()));
        }
    }
}

impl ProtocolDissector for Ieee8023EthernetDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine and extracts all the fields of a protocol on buffer of data
    **/
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree, parent_node: Option<&mut ProtoTreeNode>) -> ProtocolDissectResult {
        if self.can_dissect(buffer) {
            let eth_type: u16 = buffer.read_next_u16(12).unwrap();
            self.ether_type = EtherType::from_value(eth_type);

            let dst_mac = buffer.byte_range_checked(0, 6)
                .map(|b| b.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join(":"))
                .unwrap_or_default();

            let src_mac = buffer.byte_range_checked(6, 6)
                .map(|b| b.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join(":"))
                .unwrap_or_default();

            let fields = {
                let mut map = HashMap::new();
                map.insert("source_mac".to_string(), src_mac);
                map.insert("destination_mac".to_string(), dst_mac);
                map
            };
            let mut proto_tree_node = ProtoTreeNode::new(self.ether_type.name());
            proto_tree_node.proto_infos = fields;

            let node_ptr = {
                prototree.insert_node(proto_tree_node, parent_node) as *mut ProtoTreeNode
            };

            let node = unsafe { &mut *node_ptr };
            self.call_subsequent_dissector(buffer, prototree, node);
            Parsed

        } else {
            Error(ParseError::InvalidHeader)
        }
    }

    fn can_dissect(&self, buf: &PktBuf) -> bool {
        if buf.length < 14 {
            return false;
        }

        match buf.read_next_u16(12) {
            Some(val) => true,
            None => false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
