#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Dev,
    Prod,
}

pub const DEFAULT_MODE: Mode = Mode::Dev;