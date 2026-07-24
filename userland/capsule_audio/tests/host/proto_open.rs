#[path = "../../src/server/proto.rs"]
mod proto;
use proto::*;

fn main() {
    let req = Request {
        op: OP_STREAM_OPEN,
        request_id: 7,
        payload_len: 2,
    };
    let mut buf = [0u8; 28];
    let n = encode_open_reply(&req, E_OK, 3, &mut buf);
    assert_eq!(n, 28);
    let magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    assert_eq!(magic, MAGIC);
    let status = i32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]);
    assert_eq!(status, 0);
    let stream_id = u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]);
    assert_eq!(stream_id, 3);
    println!("HOSTTEST-PASS proto_open");
}
