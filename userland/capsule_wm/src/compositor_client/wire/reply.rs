use super::{NCMP_HDR_LEN, NCMP_MAGIC, NCMP_VERSION};

pub(super) fn decode_status(
    rx: &[u8],
    op: u16,
    request_id: u32,
    rc: i64,
) -> Result<i32, &'static str> {
    validate(rx, op, request_id, 4, rc)?;
    read_status(rx)
}
pub(super) fn decode_status_payload(
    rx: &[u8],
    op: u16,
    request_id: u32,
    body: &mut [u8],
    rc: i64,
) -> Result<i32, &'static str> {
    validate(rx, op, request_id, 4 + body.len() as u32, rc)?;
    let needed = NCMP_HDR_LEN + 4 + body.len();
    body.copy_from_slice(&rx[NCMP_HDR_LEN + 4..needed]);
    read_status(rx)
}
fn validate(
    rx: &[u8],
    op: u16,
    request_id: u32,
    payload_len: u32,
    rc: i64,
) -> Result<(), &'static str> {
    let needed = NCMP_HDR_LEN + payload_len as usize;
    if rc < needed as i64 {
        return Err("compositor call failed");
    }
    if matches_header(&rx[..needed], op, request_id, payload_len)? {
        return Ok(());
    }
    Err("compositor call failed")
}
fn matches_header(
    buf: &[u8],
    op: u16,
    request_id: u32,
    payload_len: u32,
) -> Result<bool, &'static str> {
    let magic = u32::from_le_bytes(buf[0..4].try_into().map_err(|_| "compositor short response")?);
    let version =
        u16::from_le_bytes(buf[4..6].try_into().map_err(|_| "compositor short response")?);
    let resp_op =
        u16::from_le_bytes(buf[6..8].try_into().map_err(|_| "compositor short response")?);
    let resp_request_id =
        u32::from_le_bytes(buf[12..16].try_into().map_err(|_| "compositor short response")?);
    let resp_payload_len =
        u32::from_le_bytes(buf[16..20].try_into().map_err(|_| "compositor short response")?);
    Ok(magic == NCMP_MAGIC
        && version == NCMP_VERSION
        && resp_op == op
        && resp_request_id == request_id
        && resp_payload_len == payload_len)
}
fn read_status(rx: &[u8]) -> Result<i32, &'static str> {
    Ok(i32::from_le_bytes(
        rx[NCMP_HDR_LEN..NCMP_HDR_LEN + 4].try_into().map_err(|_| "compositor short response")?,
    ))
}
