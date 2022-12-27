use crate::constant::{HEADER_LENGTH, ProtocolVersion};
use crate::header::Header;
use std::io::prelude::*;
use flate2::read::ZlibDecoder;
use crate::constant::Operation::{OP_AUTH, OP_HEARTBEAT_REPLY};

#[derive(Debug, Clone)]
pub struct RawPacket {
    header: Header,
    body: Vec<u8>,
}


impl RawPacket {
    pub fn new(data: &Vec<u8>) -> RawPacket {
        RawPacket {
            header: Header::new(&data[0.. HEADER_LENGTH].try_into().unwrap()),
            body: data.split_at(HEADER_LENGTH).1.to_vec(),
        }
    }

    /// get the body content as a String
    pub fn get_body_str(&self) -> String {
        match self.header.get_version() {
            ProtocolVersion::ZLIB => {
                panic!("Getting body str from ZLIB compressed packet is not handled right now!!!")
            },
            _ => {
                String::from_utf8(self.body.clone()).unwrap()
            }
        }

    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn to_packet(&self) -> Vec<Packet> {
        let mut result: Vec<Packet> = Vec::new();
        match self.header.get_version() {
            ProtocolVersion::ZLIB => {
                let mut decoder = ZlibDecoder::new(self.body.as_slice());
                let mut body_data:Vec<u8> = Vec::new();
                let success = decoder.read_to_end(&mut body_data);
                if !success.is_ok() {
                    return result
                }
                let mut offset = 0;
                let raw_body_data = body_data.as_slice();
                while offset < body_data.len() {
                    let packet_len = u32::from_be_bytes(raw_body_data[offset..offset+4].try_into().unwrap()) as usize;
                    let packet_data = &raw_body_data[offset..offset+packet_len];
                    result.push(Packet::from(&packet_data.to_vec()));
                    offset += packet_len;
                }
            }
            _ => {
                result.push(self.into())
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    _header: Header,
    _body: String,
}

impl Packet {

    /// NOTE: It is recommend that always using this functions in production
    /// Decodes the raw protocol message.
    /// If Header.version == 2, result will contain multiple packet.
    /// If Header.version == 0, result will only contains one packet.
    pub fn decode(data: &Vec<u8>) -> Vec<Packet> {
        RawPacket::new(data).to_packet()
    }

    #[allow(non_snake_case)]
    pub fn AuthPacket(auth_body: String) -> Packet {
        Packet {
            _header: Header {
                packet_length: (16 + auth_body.len()) as u32,
                header_length: 16,
                version: 0,
                operation: OP_AUTH as u8,
                sequence_id: 0,
            },
            _body: auth_body,
        }
    }

    #[allow(non_snake_case)]
    pub fn HeartbeatReplyPacket() -> Packet {
        Packet {
            _header: Header {
                packet_length: 16,
                header_length: 16,
                version: 0,
                operation: OP_HEARTBEAT_REPLY as u8,
                sequence_id: 0,
            },
            _body: String::new(),
        }
    }
}

impl Packet {
    pub fn new(raw_packet: &RawPacket) -> Result<Packet, Box<dyn std::error::Error>> {
        Ok(Packet{
            _header: *raw_packet.get_header(),
            _body: raw_packet.get_body_str(),
        })
    }

    /// Get the packet header
    pub fn header(&self) -> &Header {
        &self._header
    }

    pub fn body(&self) -> &String {
        &self._body
    }
}

impl From<RawPacket> for Packet {
    fn from(raw_packet: RawPacket) -> Self {
        Packet::new(&raw_packet).unwrap()
    }
}

impl From<&RawPacket> for Packet {
    fn from(raw_packet: &RawPacket) -> Self {
        Packet::new(raw_packet).unwrap()
    }
}

impl From<Packet> for RawPacket {
    fn from(packet: Packet) -> Self {
        RawPacket {
            header: *packet.header(),
            body: packet.body().clone().into_bytes(),
        }
    }
}

impl From<&Vec<u8>> for Packet {
    fn from(data: &Vec<u8>) -> Self {
        RawPacket::new(data).into()
    }
}

impl From<&[u8]> for Packet {
    fn from(raw_data: &[u8]) -> Self {
        RawPacket::new(&raw_data.to_vec()).into()
    }
}

impl From<&str> for Packet {
    fn from(raw_data: &str) -> Self {
        RawPacket::new(&raw_data.as_bytes().to_vec()).into()
    }
}

/// Encoding function for RawPacket
impl From<&RawPacket> for Vec<u8> {
    fn from(raw: &RawPacket) -> Self {
        let mut result: Vec<u8> = Vec::new();
        let mut raw_header:Vec<u8> = raw.get_header().into();
        result.append(raw_header.as_mut());
        result.append(raw.get_body().clone().as_mut());
        result
    }
}

/// Encoding function for Packet
impl From<Packet> for Vec<u8> {
    fn from(packet: Packet) -> Self {
        let mut result: Vec<u8> = Vec::new();
        let mut raw_header:Vec<u8> = packet.header().into();
        result.append(raw_header.as_mut());
        if ! packet.body().is_empty() {
            result.append(packet.body().clone().into_bytes().as_mut());
        }
        result
    }
}