use crate::epan::network::prs_arp::ARPDissector;
use crate::epan::application::prs_https::HTTPSDissector;
use crate::epan::application::prs_dns::DNSDissector;
use crate::epan::physical::prs_ipx::IPXDissector;
use crate::epan::physical::prs_llc::LLCDissector;
use crate::epan::application::prs_http::HTTPDissector;
use crate::epan::datalink::prs_ieee8023_frame::IEEE8023FrameDissector;
use crate::epan::network::prs_ipv4::IPv4Dissector;
use crate::epan::network::prs_ipv6::IPv6Dissector;
use crate::epan::physical::prs_ieee8023_ethernet::Ieee8023EthernetDissector;
use crate::epan::presentation::prs_html::HTMLDissector;
use crate::epan::presentation::prs_tls::TLSDissector;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::session::prs_ssh::SSHDissector;
use crate::epan::transport::prs_tcp::TCPDissector;
use crate::epan::transport::prs_udp::UDPDissector;

/// Here not that the order with which you place the dissectors is important
/// because for now, the dissector will be, in the first place, called sequentially

pub fn get_physical_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(Ieee8023EthernetDissector::new()),
        Box::new(LLCDissector::new()),
        Box::new(IPXDissector::new()),
   ]
}

pub fn get_datalink_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(IEEE8023FrameDissector::new()),
    ]
}

pub fn get_network_layer_parsers() -> Vec<Box<dyn ProtocolDissector>> {
    vec![
        Box::new(IPv4Dissector::new()),
        Box::new(IPv6Dissector::new()),
         Box::new(ARPDissector::new()),
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