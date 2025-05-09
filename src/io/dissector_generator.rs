use std::fs;
use std::fs::OpenOptions;
use crate::cli::dissector_gen::DissectorGenArgs;
use std::io::{Write, Read};

pub fn generate_protocol_file(args: DissectorGenArgs){
    let protocol_name = args.name.to_lowercase();
    let uppercase_protocol = protocol_name.to_uppercase();
    let parser_struct = format!("{}Dissector", uppercase_protocol);

    let file_name = format!("prs_{}.rs", protocol_name);
    let module_path = format!("src/epan/{}/{}", args.layer, file_name);
    let mod_rs_path = format!("src/epan/{}/mod.rs", args.layer);
    let layer_parser_path = "src/epan/layer_parsers.rs";

    // 1. Create parser file
    let parser_code = format!(
        "use crate::epan::protocol_dissector::ProtocolDissector;\n\
         use crate::epan::protocol_result::ProtocolDissectResult;\n\
         use crate::epan::proto_tree::ProtoTree;\n\
         use crate::io::packet_buffer::PktBuf;\n\n\
         pub struct {parser_struct} {{\
            name: &'static str
         }}\n\n\
         impl {parser_struct} {{\n\
             pub fn new() -> Self {{\n{parser_struct} {{ \n name: \"{uppercase_protocol}\"}}\n\
             }}\n\
         }}\n\n\

        fn can_dissect(&self, buffer: &PktBuf) -> bool {{
            todo!()
        }}\n\n\
         impl ProtocolDissector for {parser_struct} {{\n\
             fn protocol_dissector(&mut self, buffer: &PktBuf, prototree: &mut ProtoTree) -> ProtocolDissectResult {{\n\
                 todo!(\"Implement parser for {protocol_name}\")\n\
             }}\n\n\
             fn name(&self) -> &'static str{{self.name}}\n\n\
         }}\n"
    );
    
    fs::write(&module_path, parser_code).expect("Failed to write parser file");

    // 2. Add to mod.rs
    let mod_decl = format!("\npub mod prs_{};\n", protocol_name);
    let mut mod_file = OpenOptions::new()
        .append(true)
        .open(&mod_rs_path)
        .expect("Failed to open mod.rs");
    mod_file.write_all(mod_decl.as_bytes()).expect("Failed to write mod.rs");

    // 3. Register parser in layer_parsers.rs
    let parser_addition = format!("        Box::new({}::new()),\n", parser_struct);
    let parser_use = format!("use crate::epan::{}::prs_{}::{};\n", args.layer, protocol_name, parser_struct);

    // Read, modify, and write back layer_parsers.rs
    let mut contents = String::new();
    {
        let mut file = OpenOptions::new()
            .read(true)
            .open(layer_parser_path)
            .expect("Failed to read layer_parsers.rs");
        file.read_to_string(&mut contents).unwrap();
    }

    if !contents.contains(&parser_use) {
        contents = format!("{}{}", parser_use, contents);
    }

    // Add to parser vec (very naïve logic, can improve later)
    let target_fn = format!("get_{}_layer_parsers", args.layer);
    if let Some(pos) = contents.find(&target_fn) {
        if let Some(start_vec) = contents[pos..].find("vec![") {
            let insert_pos = pos + start_vec + contents[pos + start_vec..].find("   ]").unwrap();
            contents.insert_str(insert_pos, &parser_addition);
        }
    }

    fs::write(layer_parser_path, contents).expect("Failed to update layer_parsers.rs");

    println!("✅ Added {} dissector to {} layer! You can find it at etherpeek/src/epan/{}/prs_{}.rs", protocol_name, args.layer, args.layer, protocol_name);
}