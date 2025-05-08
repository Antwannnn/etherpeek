use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct DNSDissector {name: &'static str }

impl DNSDissector {
    pub fn new() -> Self {
    DNSDissector {
        name: "DNS"
    }
    }
}

impl ProtocolDissector for DNSDissector {
    fn protocol_dissector(&self, buffer: &PktBuf) -> ProtocolDissectResult {
        todo!("Implement parser for dns")
    }
    
    fn name(&self) -> &'static str{self.name}

}
