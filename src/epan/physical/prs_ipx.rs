use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct IPXDissector {
    name: &'static str
}

impl IPXDissector {
    pub fn new() -> Self {
        IPXDissector {
            name: "IPX"
        }
    }
}

impl ProtocolDissector for IPXDissector {
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        todo!("Implement parser for ipx")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str{
        self.name
    }
}
