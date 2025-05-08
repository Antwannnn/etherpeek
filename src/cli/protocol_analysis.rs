use clap::Args;

#[derive(Debug, Args)]
pub struct ProtocolAnalysisArgs {
    #[arg(alias = "m", short = 'm', required = false, long, value_name = "MODULE")]
    pub module: Option<String>,

    /// Specify the input file for protocol analysis
    #[arg(alias = "f", short = 'f', long, value_name = "FILE")]
    pub file: String,
}