use std::fs::File;
use std::io::{BufWriter};
use pcap::{Capture, Device, Error};
use std::io::prelude::*;
use std::time::{Duration, Instant};
use crate::io::data_format::DataFormat;
use crate::io::peek::peek::Peek;
use crate::system::ownership::transfer_file_ownership;

pub fn capture_packets(path: String, cap_duration: u16, overwrite_if_exists: bool, device: Device) {
    let device_name = device.name.clone(); // keep name before moving device
    let mut capture = Capture::from_device(device)
        .expect("Failed to start capture from device {device.name}")
        .promisc(true)
        .immediate_mode(true)
        .open()
        .unwrap_or_else(|_| {
            eprintln!("Failed to open device: {device_name}");
            std::process::exit(1);
        });


    println!("Capturing on device: {}", device_name);
    let cap_file = if overwrite_if_exists {
        File::create(&path).unwrap_or_else(|e| {
            eprintln!("Failed to overwrite file: {}", e);
            std::process::exit(1);
        })
    } else {
        match File::options()
            .write(true)
            .read(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(_) => File::open(&path).unwrap_or_else(|e| {
                eprintln!("Failed to open existing file: {}", e);
                std::process::exit(1);
            }),
        }
    };

    transfer_file_ownership(&path)
        .expect("Failed to transfer file ownership.");

    let mut buf_writer = BufWriter::new(cap_file);
    let start_time = Instant::now();
    let timeout = Duration::from_secs(cap_duration as u64);

    println!("Capturing packets for {cap_duration} seconds...");
    let mut pkt_id: u16 = 0; // Incremental
    loop {
        if Instant::now().duration_since(start_time) > timeout {
            println!("Capture completed. Saved as {path}");
            break;
        }
        match capture.next_packet() {
            Ok(packet) => {
                println!("Packet captured at {:?}", Instant::now());
                let peek_pkt = Peek::format_packet(packet, pkt_id, DataFormat::HEX);
                buf_writer.write_all(&peek_pkt.to_bytes()).expect("Failed to write into capture file");
                pkt_id += 1;
            },
            Err(Error::TimeoutExpired) => {
                // Expected if no packet is captured in time; ignore or log
                continue;
            },
            Err(e) => {
                eprintln!("Capture error: {}", e);
                break;
            }
        }
    }

    buf_writer.flush().expect("Failed to close buffer.")
}
