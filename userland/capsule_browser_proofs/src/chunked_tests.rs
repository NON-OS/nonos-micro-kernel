// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for HTTP chunked transfer decoding: hex chunk sizes and
//! CRLF framing. Many real responses arrive chunked; a wrong size parse or
//! frame skip corrupts the body.

use crate::browser::chunked::decode;

fn dec(s: &[u8]) -> alloc::vec::Vec<u8> {
    decode(s).unwrap_or_else(|| panic!("decode {s:?}"))
}

#[test]
fn decodes_chunked_bodies() {
    assert_eq!(dec(b"4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n"), b"Wikipedia", "the classic example");
    assert_eq!(dec(b"3\r\nabc\r\n0\r\n\r\n"), b"abc");
    assert_eq!(dec(b"0\r\n\r\n"), b"", "an immediate last chunk is empty");
    assert_eq!(dec(b"A\r\n0123456789\r\n0\r\n\r\n"), b"0123456789", "chunk size A is 10, hex");
    assert_eq!(dec(b"1a\r\nabcdefghijklmnopqrstuvwxyz\r\n0\r\n\r\n").len(), 26, "0x1a is 26");
}

#[test]
fn rejects_a_chunk_that_overruns_the_body() {
    // Declares 5 bytes but only 3 are present.
    assert!(decode(b"5\r\nabc\r\n0\r\n\r\n").is_none());
}
