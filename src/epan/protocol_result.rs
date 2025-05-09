use std::fmt::Display;
use crate::epan::parse_error::ParseError;

pub enum ProtocolDissectResult {
    Parsed,
    NeedsMoreData,
    Unsupported,
    Error(ParseError),
}

