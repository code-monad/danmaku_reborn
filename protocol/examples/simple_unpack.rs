use bili_danmaku_protocol::header::Header;
use bili_danmaku_protocol::packet::{Packet, RawPacket};

fn main() {
    let test_raw_header: [u8;16] = [
        0,0,0,255,
        0,16,
        0,0,
        0,0,0,2,
        0,0,0,1,
    ];

    let header  = Header::new(&test_raw_header);

    println!("{:?}", header);

    let test_raw_header2: [u8;18] = [
        0,0,0,18,
        0,16,
        0,0,
        0,0,0,2,
        0,0,0,1,
        123,125,
    ];
    let raw_packet = RawPacket::new(&test_raw_header2.to_vec());
    println!("{:?}", raw_packet);
    let packet: Packet = raw_packet.try_into().unwrap();
    println!("{:?}", packet);
}