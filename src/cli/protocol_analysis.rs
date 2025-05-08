use clap::Args;

#[derive(Debug, Args)]
pub struct ProtocolAnalysisArgs {
    /// Specify the duration of the capture (max 2^16)
    #[arg(short = 'm', long, value_name = "MODULE")]
    pub module: String,

    /// Specify the output file for captured data
    #[arg(short = 'f', long, value_name = "FILE")]
    pub file: String,
}