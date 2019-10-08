pub struct DELFHeader {
    identify: u32,
    version: u32,
    entry: u32,
    program_header_offset: u32,
    segment_header_offset: u32,
    header_size: u32,
    program_header_length: u32,
    segment_header_length: u32,
    segment_str_table_index: u32,
}

impl DELFHeader {
    fn new() -> DELFHeader {
        DELFHeader {
            identify: 0x64656c66,//delf
            version: 0,
            entry: 0,
            program_header_offset: 0,
            segment_header_offset: 0,
            header_size: 64,
            program_header_length: 0,
            segment_header_length: 0,
            segment_str_table_index: 0,
        }
    }

    fn encode_to_bytes(&self) -> Vec<u8> {
        return Vec::new();
    }

    fn decode_from_bytes(headers: Vec<u8>) -> DELFHeader {
         DELFHeader{
             identify: 0,
             version: 0,
             entry: 0,
             program_header_offset: 0,
             segment_header_offset: 0,
             header_size: 0,
             program_header_length: 0,
             segment_header_length: 0,
             segment_str_table_index: 0
         }
    }
}


