//! The framing faults that let Sphinx packets vanish between two capsules.

use crate::protocol::errno::E_BAD_LEN;
use crate::server::parse_req::{parse, HDR_LEN, IPC_BUF_MAX, SEGMENT_PAYLOAD_MAX};
use crate::server::recv_cap::recv_cap;

/// Build a request frame carrying `payload`.
fn frame(op: u16, request_id: u32, payload: &[u8]) -> Vec<u8> {
    let mut buf = vec![0u8; HDR_LEN + payload.len()];
    buf[0..4].copy_from_slice(&0x4e544350u32.to_le_bytes());
    buf[4..6].copy_from_slice(&1u16.to_le_bytes());
    buf[6..8].copy_from_slice(&op.to_le_bytes());
    buf[12..16].copy_from_slice(&request_id.to_le_bytes());
    buf[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    buf[HDR_LEN..].copy_from_slice(payload);
    buf
}

/// The regression itself. A send of a full segment carries the socket handle
/// as well, and the inbox has to hold both or the request never parses.
#[test]
fn a_full_segment_send_fits_the_inbox() {
    let payload = vec![0xa5u8; 4 + SEGMENT_PAYLOAD_MAX];
    let req = frame(5, 7, &payload);
    assert!(req.len() <= HDR_LEN + IPC_BUF_MAX, "inbox too small for a full send");
    let (parsed, body) = parse(&req).expect("a full segment send must parse");
    assert_eq!(parsed.request_id, 7);
    assert_eq!(body.len(), payload.len());
}

/// What a truncated read looks like from the far side: the header still
/// claims the full length, so the shortfall is visible rather than silent.
#[test]
fn a_truncated_request_is_refused_not_read_short() {
    let payload = vec![0u8; 4 + SEGMENT_PAYLOAD_MAX];
    let req = frame(5, 9, &payload);
    let cut = &req[..1044];
    assert_eq!(parse(cut).err(), Some(E_BAD_LEN));
}

/// A refusal has to be addressable, which means reading the header back out
/// of bytes that failed to parse as a whole request.
#[test]
fn a_refusable_request_still_carries_its_reply_fields() {
    let req = frame(5, 0xdeadbeef, &vec![0u8; 64]);
    let cut = &req[..HDR_LEN];
    assert!(parse(cut).is_err());
    assert_eq!(u16::from_le_bytes([cut[6], cut[7]]), 5);
    assert_eq!(u32::from_le_bytes([cut[12], cut[13], cut[14], cut[15]]), 0xdeadbeef);
}

/// A caller that states its capacity is held to it, because a read consumes
/// what it copies and the remainder cannot be asked for again.
#[test]
fn a_stated_capacity_bounds_the_drain() {
    let mut body = [0u8; 8];
    body[4..8].copy_from_slice(&512u32.to_le_bytes());
    assert_eq!(recv_cap(&body), 512);
}

/// An oversized claim cannot talk the server into a reply the caller could
/// not hold either.
#[test]
fn a_stated_capacity_cannot_exceed_the_inbox() {
    let mut body = [0u8; 8];
    body[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(recv_cap(&body), IPC_BUF_MAX);
}

/// A caller from before the field keeps the size it was built against, so
/// widening the inbox does not start overrunning it.
#[test]
fn a_caller_without_the_field_keeps_the_old_size() {
    assert_eq!(recv_cap(&[0u8; 4]), 1024);
}

/// Zero would allocate nothing and report an empty socket forever.
#[test]
fn a_zero_capacity_still_leaves_room_to_read() {
    let body = [0u8; 8];
    assert!(recv_cap(&body) >= 1);
}
