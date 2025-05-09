use crate::epan::proto_tree::ProtoTree;
use crate::epan::protocol_dissector::ProtocolDissector;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

pub struct HTTPSDissector {name: &'static str
         }

impl HTTPSDissector {
    pub fn new() -> Self {
        HTTPSDissector { 
         name: "HTTPS"}
        }
    }

impl ProtocolDissector for HTTPSDissector {
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult {
        todo!("Implement parser for https")
    }

    fn can_dissect(&self, buffer: &PktBuf) -> bool {
        todo!()
    }

    fn name(&self) -> &'static str{self.name}

}
