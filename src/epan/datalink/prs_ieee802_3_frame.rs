use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;
use crate::io::protocol::Protocol;

pub struct IEEE802_3FrameDissector {

    name: &'static str

}

impl IEEE802_3FrameDissector {
    pub fn new() -> Self {
        IEEE802_3FrameDissector {
            name: "Ethernet IEEE802.3 Frame (MAC)"
        }
    }
}

impl ProtocolDissector for IEEE802_3FrameDissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine which protocol is
    **/
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        todo!("Implement parser for ieee802.3 frame")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}