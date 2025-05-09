use std::collections::HashMap;
use crate::epan::osi::OsiLayer;
use crate::epan::parse_error::ParseError;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::epan::protocol_result::ProtocolDissectResult::{Error, Parsed};
use crate::io::packet_buffer::PktBuf;
use crate::io::protocol::Protocol;

pub struct IEEE802_3Dissector {

    name: &'static str

}

impl IEEE802_3Dissector {
    pub fn new() -> Self {
        IEEE802_3Dissector {
            name: "Ethernet II"
        }
    }
}

impl ProtocolDissector for IEEE802_3Dissector {

    /**
    This represents the main function where the buffer of data is going to be analyzed
    to determine and extracts all the fields of a protocol on buffer of data
    **/
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        if self.can_dissect(buffer) {
            return Parsed(Protocol::new(
                self.name.to_string(),
                OsiLayer::Physical,
                Some(0x01),
                HashMap::new(),
                buffer.buf.clone(),
                0xFF
            ));
        }

        Error(ParseError::InvalidHeader)

    }

    fn can_dissect(&self, buf: &PktBuf) -> bool {
        if buf.length < 14 {
            return false;
        }

        match buf.read_next_u16(12) {
            Some(val) => val >= 1500,
            None => false,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}