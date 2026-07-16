// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for base64 decoding against the canonical RFC 4648
//! vectors. data: URIs carry inline images and fonts as base64; a wrong pad or
//! bit-shuffle corrupts them.
use super::base64::decode_b64;

fn dec(s: &str) -> alloc::vec::Vec<u8> {
    decode_b64(s).unwrap_or_else(|| panic!("decode {s:?}"))
}

#[test]
fn rfc4648_test_vectors() {
    assert_eq!(dec(""), b"");
    assert_eq!(dec("Zg=="), b"f");
    assert_eq!(dec("Zm8="), b"fo");
    assert_eq!(dec("Zm9v"), b"foo");
    assert_eq!(dec("Zm9vYg=="), b"foob");
    assert_eq!(dec("Zm9vYmE="), b"fooba");
    assert_eq!(dec("Zm9vYmFy"), b"foobar");
    assert_eq!(dec("TWFu"), b"Man");
}

#[test]
fn whitespace_is_skipped_and_bad_chars_rejected() {
    assert_eq!(dec("Zm9v\n Ym Fy"), b"foobar", "newlines and spaces are ignored");
    assert!(decode_b64("Zm9v*").is_none(), "an illegal character fails");
}
