use clap::Parser;
use crate::cli::commands::{Cli, Commands};
use crate::epan::peek_reader::read_cap_file;
use crate::io::dissector_generator::generate_protocol_file;
use crate::libcap::device::select_capture_device;
use crate::libcap::packet_capture;

mod environment;
mod cli;
mod io;
mod system;
mod libcap;
mod epan;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Capture(args) => {
            let device = args.device.unwrap_or_else(select_capture_device);
            packet_capture::capture_packets(args.file + ".peek", args.duration, true, device);
        }
        Commands::Pan(args) => {
            read_cap_file(args.file);
        }
        Commands::DvList(_) => {
            libcap::device::list_net_interfaces();
        },
        Commands::DissectorGen(args) => {
            generate_protocol_file(args);
        }
    }
}
