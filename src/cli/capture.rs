use clap::Args;
use pcap::Device;
use crate::io::data_format::DataFormat;

#[derive(Debug, Args)]
pub struct CaptureArgs {
    /// Specify the duration of the capture (max 2^16)
    #[arg(short = 'd', alias = "d", long, value_name = "DURATION", help = "Specify the capture duration.")]
    pub duration: u16,

    /// Specify the output file for captured data
    #[arg(short = 'f', alias = "f", long, value_name = "FILE", help = "Specify the capture file path.")]
    pub file: String,

    /// Specify the data format in which you want to write on the file
    #[arg(long, alias = "df", default_value = "ascii", required = false, value_name = "DATA_FORMAT", help = "Specify the capture file data format.")]
    pub dataformat: DataFormat, // ou data_format::DataFormat si tu veux parser direct vers enum

    #[arg(long, alias = "dv", required = false, value_name = "DEVICE_NAME", help = "Specify the network device you want to capture. Will choose the best by default.")]
    pub device: Option<Device>
}