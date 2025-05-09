use crate::epan::osi::OsiLayer;
use std::collections::HashMap;
use std::fmt::{format, Display, Formatter};

pub struct Protocol {
    pub name: String,
    pub layer: OsiLayer,
    pub identifier: Option<u16>,
    pub fields: HashMap<String, String>, 
    pub raw_data: Vec<u8>, // Storing the raw buffer containing the header and the payload
    pub header_len: usize, // length of the header
}

impl Protocol{
    
    pub fn new(name: String, 
               layer: OsiLayer, 
               identifier: Option<u16>,
               fields: HashMap<String, String>,
               raw_data: Vec<u8>,
               header_len: usize
    ) -> Self {
        println!("creating protocol {name}");
        Protocol{
            name,
            layer,
            identifier,
            fields,
            raw_data,
            header_len
        }
    }
    pub fn payload_offset(&self) -> usize {
        self.raw_data.len() - self.header_len
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let header_len = &self.header_len;
        let id = &self.identifier.unwrap();
        let layer = &self.layer.as_str();
        let str = format!("------ Protocol: {name} ------\n\
                Header Length: {header_len} \n\
                Identifier: {id} \n\
                Layer: {layer} \n\
                ").to_string();
        write!(f, "{}", str)
    }
}