//! The frame boundary a mixnet write has to be split on.

use crate::mixnet_frame::{decode, encode, MAX_BODY};

const IP: [u8; 4] = [185, 186, 147, 240];
const PORT: u16 = 443;

/// The size the chunker splits on has to be one the encoder will take, or
/// every full chunk is refused and only the last short one goes out.
#[test]
fn a_full_body_encodes() {
    let body = vec![0x5au8; MAX_BODY];
    let frame = encode(IP, PORT, &body).expect("a full body must encode");
    let back = decode(&frame).expect("its own frame must decode");
    assert_eq!(back.body, &body[..]);
    assert_eq!(back.ip, IP);
    assert_eq!(back.port, PORT);
}

/// One byte past it is refused, which is what made an unsplit write fail.
#[test]
fn a_body_past_the_limit_is_refused() {
    assert!(encode(IP, PORT, &vec![0u8; MAX_BODY + 1]).is_none());
}

/// A TLS record is several frames. Walking it the way the sender does has to
/// leave every piece encodable and reassemble to the original.
#[test]
fn a_tls_sized_write_splits_into_encodable_frames() {
    let payload: Vec<u8> = (0..4096u32).map(|i| (i % 251) as u8).collect();
    let mut sent = 0usize;
    let mut seen = Vec::new();
    while sent < payload.len() {
        let end = (sent + MAX_BODY).min(payload.len());
        let frame = encode(IP, PORT, &payload[sent..end]).expect("every chunk must encode");
        seen.extend_from_slice(decode(&frame).expect("every chunk must decode").body);
        sent = end;
    }
    assert_eq!(seen, payload, "the far end must see the write it was given");
    assert!(payload.len() > MAX_BODY, "this record has to be worth splitting");
}

/// An empty write is still a frame. Dropping it would lose a half close.
#[test]
fn an_empty_write_is_still_a_frame() {
    let frame = encode(IP, PORT, &[]).expect("an empty body must encode");
    assert!(decode(&frame).expect("an empty frame must decode").body.is_empty());
}

/// A frame claiming more body than it carries cannot be read past its end.
#[test]
fn a_frame_claiming_more_than_it_carries_is_refused() {
    let mut frame = encode(IP, PORT, &[1, 2, 3, 4]).unwrap();
    frame[14..16].copy_from_slice(&4096u16.to_le_bytes());
    assert!(decode(&frame).is_none());
}
