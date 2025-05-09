use crate::epan::proto_tree::ProtoTree;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;

/// A high-level trait that all protocol parsers implement.
/// Intended to be dispatched at runtime when analyzing a packet.
pub trait ProtocolDissector {
    /// Parses the given packet buffer. The implementation is expected to
    /// either fully handle or dispatch to the appropriate next-layer parser.
    fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult;
    
    fn can_dissect(&self, buffer: &PktBuf) -> bool;

    /// Returns the protocol name (useful for debugging/logging).
    fn name(&self) -> &'static str;
    
}