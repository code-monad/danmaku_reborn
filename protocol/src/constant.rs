#[allow(non_camel_case_types)] // Keeps the same name with the api doc
#[repr(u8)]
pub enum Operation {
    OP_HEARTBEAT = 2,
    OP_HEARTBEAT_REPLY = 3,
    OP_SEND_SMS_REPLY = 5,
    OP_AUTH = 7,
    OP_AUTH_REPLY = 8,
}

/// Header always in 16 bytes
pub const HEADER_LENGTH: usize = 16;

/// Packet Length in byte index 0-3
pub const PACKET_LENGTH_FIELD: (usize,usize) = (0,4);

/// Header Length in byte index 4-5
/// NOTE: It's value always equals to 16(bytes)
pub const HEADER_LENGTH_FIELD: (usize,usize) = (4,6);

/// Version field in byte index 6-7
/// NOTE: It's value can be `0` or `2`
pub const VERSION_FIELD: (usize,usize) = (6,8);

/// Operation field, check enum Operation
pub const OPERATION_FIELD: (usize,usize) = (8, 12);

/// Not sure what is this field used for
pub const SEQUENCE_ID_FIELD: (usize,usize) = (12, 16);


/// Version field names, not from bilibili's doc but for convenient
#[repr(u8)]
pub enum ProtocolVersion {
    RAW = 0, // body content as-is
    ZLIB = 2, // must use zlib to extract first
    UNKNOWN,
}

impl From<u8> for ProtocolVersion {
    fn from(val: u8) -> Self {
        match val {
            0 => ProtocolVersion::RAW,
            2 => ProtocolVersion::ZLIB,
            _ => ProtocolVersion::UNKNOWN
        }
    }
}