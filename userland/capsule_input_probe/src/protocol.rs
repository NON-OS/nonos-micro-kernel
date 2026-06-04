use nonos_libc::InputEvent;

pub const REQ_MAGIC: u32 = 0x4E49_5253; // "NIRS"
pub const REQ_VERSION: u16 = 1;
pub const REQ_HDR_LEN: usize = 20;

pub const OP_SUBSCRIBE: u16 = 0x0002;
pub const OP_GRAB_REQUEST: u16 = 0x0003;

pub const DELIVERY_MAGIC: u32 = 0x4E49_4E50; // "NINP"
pub const DELIVERY_VERSION: u16 = 1;
pub const DELIVERY_HDR_LEN: usize = 8;
pub const DELIVERY_LEN: usize = DELIVERY_HDR_LEN + core::mem::size_of::<InputEvent>();

pub fn encode_request(out: &mut [u8], op: u16, request_id: u32, body: &[u8]) -> usize {
    out[0..4].copy_from_slice(&REQ_MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&REQ_VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].fill(0);
    out[10..12].fill(0);
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    out[REQ_HDR_LEN..REQ_HDR_LEN + body.len()].copy_from_slice(body);
    REQ_HDR_LEN + body.len()
}

pub fn parse_delivery(buf: &[u8]) -> Option<InputEvent> {
    if buf.len() < DELIVERY_LEN {
        return None;
    }
    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    if magic != DELIVERY_MAGIC {
        return None;
    }
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    if version != DELIVERY_VERSION {
        return None;
    }
    Some(InputEvent {
        kind: u16::from_le_bytes(buf[8..10].try_into().ok()?),
        flags: u16::from_le_bytes(buf[10..12].try_into().ok()?),
        code: u32::from_le_bytes(buf[12..16].try_into().ok()?),
        x: i32::from_le_bytes(buf[16..20].try_into().ok()?),
        y: i32::from_le_bytes(buf[20..24].try_into().ok()?),
        delta_x: i32::from_le_bytes(buf[24..28].try_into().ok()?),
        delta_y: i32::from_le_bytes(buf[28..32].try_into().ok()?),
        timestamp_ns: u64::from_le_bytes(buf[32..40].try_into().ok()?),
    })
}
