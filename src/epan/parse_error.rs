pub enum ParseError {
    InvalidHeader,
    UnsupportedProtocol,
    TruncatedPacket,
    Io(std::io::Error),
}