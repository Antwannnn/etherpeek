use crate::epan::parse_error::ParseError;
use crate::io::protocol::Protocol;

pub enum ProtocolDissectResult {
    Parsed(Protocol),
    NeedsMoreData,
    Unsupported,
    Error(ParseError),
}