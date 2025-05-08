use crate::epan::osi::OsiLayer;
use std::collections::HashMap;

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