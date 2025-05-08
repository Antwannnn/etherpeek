use clap::{Parser, Subcommand};
use crate::cli::capture::CaptureArgs;
use crate::cli::dissector_gen::DissectorGenArgs;
use crate::cli::protocol_analysis::ProtocolAnalysisArgs;
use crate::cli::dvlist::DeviceListArgs;

#[derive(Parser)]
#[command(name = "etherpeek", version = "0.1", author = "antoine.leboucher@icloud.com", about = "Capture your network traffic")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Allows you to capture your incoming network packet.
    Capture(CaptureArgs),

    /// Allows you to launch a protocol analysis on a specified file
    Pan(ProtocolAnalysisArgs),

    // Allows you to easily list your network device
    DvList(DeviceListArgs),
    
    DissectorGen(DissectorGenArgs)
}