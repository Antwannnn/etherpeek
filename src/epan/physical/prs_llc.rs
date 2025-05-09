use crate::epan::parse_error::ParseError::UnsupportedProtocol;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::epan::protocol_result::ProtocolDissectResult::Error;
use crate::io::packet_buffer::PktBuf;

pub struct LLCDissector {
    name: &'static str
}
impl LLCDissector {
    pub fn new() -> Self {
        LLCDissector { 
            name: "LLC"
        }
    }
}


impl ProtocolDissector for LLCDissector {
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        Error(UnsupportedProtocol)
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

