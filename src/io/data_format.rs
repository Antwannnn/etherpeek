use std::str::FromStr;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DataFormat{
    HEX,
    BINARY,
    ASCII
}

impl DataFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataFormat::HEX => "hex",
            DataFormat::BINARY => "bin",
            DataFormat::ASCII => "ascii",
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            DataFormat::HEX => 1,
            DataFormat::BINARY => 2,
            DataFormat::ASCII => 3,
        }
    }

    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(DataFormat::HEX),
            2 => Some(DataFormat::BINARY),
            3 => Some(DataFormat::ASCII),
            _ => None,
        }
    }
}


impl FromStr for DataFormat {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "hex" => Ok(DataFormat::HEX),
            "bin" | "binary" => Ok(DataFormat::BINARY),
            "ascii" => Ok(DataFormat::ASCII),
            _ => Err(()),
        }
    }
}