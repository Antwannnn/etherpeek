use std::collections::HashMap;
use std::fmt::format;
use crate::epan::osi::OsiLayer;
use crate::epan::parse_error::ParseError;
use crate::epan::physical::prs_ieee8023_ethernet::EtherType::{EthernetII, IEEE8023};
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node::ProtoTreeNode;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::epan::protocol_result::ProtocolDissectResult::{Error, Parsed};
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

    pub fn ether_type_description(val: u16) -> &'static str {
        match val {
            0x0800 => "IPv4",
            0x0806 => "ARP",
            0x86DD => "IPv6",
            _ => "Unknown",
        }
    }
}

impl SubDissectorCaller for Ieee8023EthernetDissector {
    fn call_subsequent_dissector(pkt_buf: &PktBuf) {
        todo!()
    }
}

impl ProtocolDissector for Ieee8023EthernetDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine and extracts all the fields of a protocol on buffer of data
    **/
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult {
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
                map.insert("ethertype".to_string(), format!("0x{:04X}", eth_type));
                map.insert("ethertype_description".to_string(), Self::ether_type_description(eth_type).to_string());
                map.insert("source_mac".to_string(), src_mac);
                map.insert("destination_mac".to_string(), dst_mac);
                map
            };
            let mut proto_tree_node = ProtoTreeNode::new(self.ether_type.name());
            proto_tree_node.proto_infos = fields;
            prototree.nodes.push(proto_tree_node);
            
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
            // When having a value at this byte, it is necessarily ethernet
            Some(val) => true,
            None => false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
