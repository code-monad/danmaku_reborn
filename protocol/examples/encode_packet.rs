use bili_danmaku_protocol::packet::{Packet};

fn main() {
    //generates a fake auth packet
    let packet = Packet::AuthPacket(String::from("PRETEND_ME_AS_A_AUTH_BODY"));
    let encoded_bytes: Vec<u8> = packet.clone().into(); // encodes into a bytes array
    println!("Original OP_AUTH Packet:{:?}\nEncoded:{:?}", packet, encoded_bytes);
    //generates a fake heartbeat reply packet
    let hb_reply_packet = Packet::HeartbeatReplyPacket();
    let encoded_bytes: Vec<u8> = hb_reply_packet.clone().into(); // encodes into a bytes array
    println!("Original Heartbeat Reply Packet:{:?}\nEncoded:{:?}", hb_reply_packet, encoded_bytes);
}