use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct DissectorGenArgs {
    /// Protocol name (e.g., ssh, ip, tcp)
    #[arg(short, long)]
    pub name: String,

    /// OSI layer to place this protocol in (e.g., physical, network)
    #[arg(short, long)]
    pub layer: String,
}