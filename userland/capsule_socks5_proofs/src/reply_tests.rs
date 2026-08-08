// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that the proxy can say "nothing yet". The kernel refuses a
//! zero-length reply, so an answer carrying no stream bytes still has to be a
//! real message: every reply leads with a marker, an empty open reply is one
//! byte long, and the marker says whether the tunnel is still open.

use crate::reply::{Reply, STREAM_CLOSED, STREAM_OPEN};

#[test]
fn an_empty_answer_is_still_a_message() {
    let encoded = Reply::open(Vec::new()).encode();
    assert_eq!(encoded, vec![STREAM_OPEN], "one byte, never zero");
    assert!(!encoded.is_empty(), "the kernel refuses a zero length reply");
}

#[test]
fn stream_bytes_follow_the_marker() {
    let encoded = Reply::open(b"HTTP/1.1 200 OK".to_vec()).encode();
    assert_eq!(encoded[0], STREAM_OPEN);
    assert_eq!(&encoded[1..], b"HTTP/1.1 200 OK");
}

#[test]
fn a_close_is_carried_with_its_last_bytes() {
    let encoded = Reply::closed(b"tail".to_vec()).encode();
    assert_eq!(encoded[0], STREAM_CLOSED);
    assert_eq!(&encoded[1..], b"tail");
}

#[test]
fn a_close_with_nothing_left_is_one_byte() {
    assert_eq!(Reply::closed(Vec::new()).encode(), vec![STREAM_CLOSED]);
}

#[test]
fn the_two_markers_are_distinct() {
    // The reader decides whether to ask again on this single byte, so the
    // two states cannot share a value.
    assert_ne!(STREAM_OPEN, STREAM_CLOSED);
}

// The same problem exists on the way in: the kernel refuses a zero length
// message, so a request carrying no stream bytes needs a byte to travel on.
use crate::request::{stream_bytes, STREAM_BYTES};

#[test]
fn a_request_with_no_stream_bytes_is_still_sendable() {
    let framed = [STREAM_BYTES];
    assert_eq!(
        stream_bytes(&framed),
        Some(&[][..]),
        "a poll carries nothing and is still a message"
    );
}

#[test]
fn stream_bytes_are_recovered_untouched() {
    let mut framed = vec![STREAM_BYTES];
    framed.extend_from_slice(b"\x05\x01\x00");
    assert_eq!(
        stream_bytes(&framed),
        Some(&b"\x05\x01\x00"[..]),
        "the SOCKS greeting arrives whole"
    );
}

#[test]
fn an_unmarked_request_is_refused() {
    // Bare SOCKS bytes, as the browser used to send them. The greeting opens
    // with 0x05, which is not a marker this speaks.
    assert!(stream_bytes(&[0x05, 0x01, 0x00]).is_none());
    assert!(stream_bytes(&[]).is_none(), "there is no empty request to read");
}

use crate::request::{ask, Ask, STREAM_RESET};

#[test]
fn a_reset_is_told_apart_from_stream_bytes() {
    // The greeting is three bytes and would relay as data to a connection
    // that is already open, which is how every page after the first failed.
    assert!(matches!(ask(&[STREAM_RESET]), Some(Ask::Reset)));
    let mut greeting = vec![0u8];
    greeting.extend_from_slice(b"\x05\x01\x00");
    match ask(&greeting) {
        Some(Ask::Stream(bytes)) => assert_eq!(bytes, b"\x05\x01\x00"),
        _ => panic!("a marked greeting is stream bytes, not a reset"),
    }
}

#[test]
fn an_unknown_marker_is_refused() {
    assert!(ask(&[9, 1, 2]).is_none());
    assert!(ask(&[]).is_none());
}
