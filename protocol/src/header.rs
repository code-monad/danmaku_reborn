use crate::constant::*;

/// Header bytes before unpacked
type RawHeader = [u8;16];

/// Unpacked header
#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub packet_length : u32,
    pub header_length : u8,
    pub version: u8,
    pub operation: u8,
    pub sequence_id: u32,
}

impl Header {
    pub fn new(raw_header: &RawHeader) -> Header {
        Header {
            packet_length: u32::from_be_bytes(raw_header.get(PACKET_LENGTH_FIELD.0..PACKET_LENGTH_FIELD.1).unwrap().try_into().unwrap()),
            header_length: 16,
            version: u16::from_be_bytes(raw_header.get(HEADER_LENGTH_FIELD.0..HEADER_LENGTH_FIELD.1).unwrap().try_into().unwrap()).try_into().unwrap(),
            operation: u32::from_be_bytes(raw_header.get(OPERATION_FIELD.0..OPERATION_FIELD.1).unwrap().try_into().unwrap()).try_into().unwrap(),
            sequence_id: u32::from_be_bytes(raw_header.get(SEQUENCE_ID_FIELD.0..SEQUENCE_ID_FIELD.1).unwrap().try_into().unwrap()),
        }
    }

    pub fn get_version(&self) -> ProtocolVersion {
        self.version.into()
    }

}

impl From<Header> for RawHeader {
    fn from(header: Header) -> Self {
        [
            ((header.packet_length >> 24) & 0xff) as u8,
            ((header.packet_length >> 16) & 0xff) as u8,
            ((header.packet_length >> 8) & 0xff) as u8,
            (header.packet_length& 0xff) as u8,
            0, 16,
            0, header.version,
            0, 0, 0, header.operation,
            ((header.sequence_id >> 24) & 0xff) as u8,
            ((header.sequence_id >> 16) & 0xff) as u8,
            ((header.sequence_id >> 8) & 0xff) as u8,
            (header.sequence_id & 0xff) as u8,
        ]
    }
}

impl From<&Header> for Vec<u8> {
    fn from(header: &Header) -> Self {
        [
            header.packet_length.to_be_bytes(),
            [0, 16, 0, header.version,],
            [0, 0, 0, header.operation,],
            header.sequence_id.to_be_bytes()
        ].concat()
    }
}