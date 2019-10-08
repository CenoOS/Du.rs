const ELF_HEADER_PREFIX: [u8; 4] = [0x64, 0x65, 0x6c, 0x66];
const ELF_HEADER_LENGTH: u32 = 64;

pub struct DELFHeader {
    identify: [u8; 4],
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
            identify: ELF_HEADER_PREFIX,
            version: 0,
            entry: 0,
            program_header_offset: 0,
            segment_header_offset: 0,
            header_size: ELF_HEADER_LENGTH,
            program_header_length: 0,
            segment_header_length: 0,
            segment_str_table_index: 0,
        }
    }

    fn encode_to_bytes(&self) -> Vec<u8> {
        return Vec::new();
    }

    fn decode_from_bytes(headers: Vec<u8>) -> DELFHeader {
        DELFHeader {
            identify: ELF_HEADER_PREFIX,
            version: 0,
            entry: 0,
            program_header_offset: 0,
            segment_header_offset: 0,
            header_size: ELF_HEADER_LENGTH,
            program_header_length: 0,
            segment_header_length: 0,
            segment_str_table_index: 0,
        }
    }
}


