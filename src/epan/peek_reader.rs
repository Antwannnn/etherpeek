use std::fs::File;
use std::io::{BufReader, Read};
use crate::epan::layer_parsers::get_physical_layer_parsers;
use crate::epan::proto_tree::ProtoTree;
use crate::epan::proto_tree_node;
use crate::epan::protocol_result::ProtocolDissectResult;
use crate::io::packet_buffer::PktBuf;
use crate::io::peek::peek::Peek;

pub fn read_cap_file(path: String) -> Vec<PktBuf> {
    let cap_file = File::open(&path).unwrap_or_else(|_| {
        eprintln!("File doesn't exist");
        std::process::exit(1);
    });

    let mut reader = BufReader::new(cap_file);
    let mut packets = Vec::new();

    loop {
        let mut header_buf = [0u8; 13];
        if let Err(_) = reader.read_exact(&mut header_buf) {
            break;
        }

        let length = u16::from_le_bytes([header_buf[8], header_buf[9]]) as usize;
        let mut full_packet = Vec::with_capacity(13 + length);
        full_packet.extend_from_slice(&header_buf);
        let mut payload_buf = vec![0u8; length];
        if let Err(_) = reader.read_exact(&mut payload_buf) {
            eprintln!("Unexpected EOF while reading packet payload");
            break;
        }
        full_packet.extend_from_slice(&payload_buf);

        // Step 5: Parse it
        match Peek::from_bytes(&full_packet) {
            Ok(peek_pkt) => {
                println!("{}", peek_pkt.to_string());
                let pkt_buf = PktBuf::new(peek_pkt);
                let proto_tree_node = call_cascading_dissector(&pkt_buf);
                packets.push(pkt_buf);
                println!("{}", proto_tree_node)
            }
            Err(e) => {
                eprintln!("Failed to parse Peek packet");
                break;
            }
        }
    }
    packets
}

pub fn call_cascading_dissector(buffer: &PktBuf) -> ProtoTree {
    let mut prototree = ProtoTree::new();
    get_physical_layer_parsers().iter_mut().for_each(|dissector| {
        dissector.protocol_dissector(buffer, &mut prototree, None);
    });

    prototree
}