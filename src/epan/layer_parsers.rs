use crate::epan::application::prs_https::HTTPSDissector;
use crate::epan::application::prs_dns::DNSDissector;
use crate::epan::physical::prs_ipx::IPXDissector;
use crate::epan::physical::prs_llc::LLCDissector;
use crate::epan::application::HTTPDissector;
use crate::epan::datalink::IEEE802_3FrameDissector;
use crate::epan::network::{IPV4Dissector, IPV6Dissector};
use crate::epan::physical::*;
use crate::epan::presentation::{HTMLDissector, TLSDissector};
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::session::SSHDissector;
use crate::epan::transport::*;

/// Here not that the order with which you place the dissectors is important
/// because for now, the dissector will be, in the first place, called sequentially

pub fn get_physical_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(IEEE802_3Dissector::new()),
        Box::new(LLCDissector::new()),
        Box::new(IPXDissector::new()),
   ]
}

pub fn get_datalink_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(IEEE802_3FrameDissector::new()),
    ]
}

pub fn get_network_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(IPV4Dissector::new()),
        Box::new(IPV6Dissector::new()),

    ]
}

pub fn get_transport_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(TCPDissector::new()),
        Box::new(UDPDissector::new()),
    ]
}

pub fn get_session_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(SSHDissector::new()),
    ]
}

pub fn get_presentation_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(HTMLDissector::new()),
        Box::new(TLSDissector::new()),
    ]
}

pub fn get_application_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(HTTPDissector::new()),
        Box::new(DNSDissector::new()),
        Box::new(HTTPSDissector::new()),
   ]
}