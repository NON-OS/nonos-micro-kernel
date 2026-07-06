// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;

use crate::vfs_protocol::{
    decode_request, encode_response, DecodeError, HDR_LEN, MAGIC, MAX_PAYLOAD_BYTES, OP_OPEN,
    OP_STAT, VERSION,
};

// Build request bytes with an explicitly chosen declared payload length, so a
// mismatch between the declared and actual payload can be exercised.
fn raw(magic: u32, version: u16, op: u16, declared_len: u32, payload: &[u8]) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&magic.to_le_bytes());
    b.extend_from_slice(&version.to_le_bytes());
    b.extend_from_slice(&op.to_le_bytes());
    b.extend_from_slice(&7u16.to_le_bytes()); // flags
    b.extend_from_slice(&0u16.to_le_bytes()); // reserved
    b.extend_from_slice(&42u32.to_le_bytes()); // request_id
    b.extend_from_slice(&declared_len.to_le_bytes());
    b.extend_from_slice(payload);
    b
}

#[test]
fn decodes_a_well_formed_request() {
    let bytes = raw(MAGIC, VERSION, OP_OPEN, 3, b"abc");
    let req = decode_request(&bytes).unwrap();
    assert_eq!(req.op, OP_OPEN);
    assert_eq!(req.flags, 7);
    assert_eq!(req.request_id, 42);
    assert_eq!(req.payload, b"abc");
}

#[test]
fn rejects_short_buffers() {
    assert!(matches!(decode_request(&[]), Err(DecodeError::Short)));
    assert!(matches!(decode_request(&[0u8; 19]), Err(DecodeError::Short)));
}

#[test]
fn rejects_bad_magic() {
    let bytes = raw(0xDEAD_BEEF, VERSION, OP_OPEN, 0, b"");
    assert!(matches!(decode_request(&bytes), Err(DecodeError::BadMagic)));
}

#[test]
fn rejects_bad_version() {
    let bytes = raw(MAGIC, 0xABCD, OP_OPEN, 0, b"");
    assert!(matches!(decode_request(&bytes), Err(DecodeError::BadVersion)));
}

#[test]
fn rejects_oversized_payload_length() {
    let bytes = raw(MAGIC, VERSION, OP_OPEN, MAX_PAYLOAD_BYTES + 1, b"");
    assert!(matches!(decode_request(&bytes), Err(DecodeError::BadLength)));
}

#[test]
fn rejects_declared_longer_than_actual() {
    // Header claims 100 payload bytes but only 3 are present.
    let bytes = raw(MAGIC, VERSION, OP_OPEN, 100, b"abc");
    assert!(matches!(decode_request(&bytes), Err(DecodeError::BadLength)));
}

#[test]
fn giant_declared_length_does_not_overflow() {
    // u32::MAX declared length must be rejected cleanly, never panic on the
    // HDR_LEN + payload_len addition.
    let bytes = raw(MAGIC, VERSION, OP_OPEN, u32::MAX, b"");
    assert!(matches!(decode_request(&bytes), Err(DecodeError::BadLength)));
}

#[test]
fn empty_payload_at_exact_header_length_is_valid() {
    let bytes = raw(MAGIC, VERSION, OP_STAT, 0, b"");
    assert_eq!(bytes.len(), HDR_LEN);
    let req = decode_request(&bytes).unwrap();
    assert_eq!(req.op, OP_STAT);
    assert!(req.payload.is_empty());
}

#[test]
fn trailing_bytes_beyond_declared_are_ignored() {
    // Declared 3 payload bytes, buffer carries 5; only the declared window is
    // exposed, never the extra bytes.
    let bytes = raw(MAGIC, VERSION, OP_OPEN, 3, b"abcXX");
    let req = decode_request(&bytes).unwrap();
    assert_eq!(req.payload, b"abc");
}

#[test]
fn encode_response_frames_header_status_and_body() {
    let out = encode_response(OP_STAT, 0, 5, 0, &[1, 2, 3]);
    // magic, then payload_len = 4 (status) + 3 (body) = 7.
    assert_eq!(&out[0..4], &MAGIC.to_le_bytes());
    let payload_len = u32::from_le_bytes([out[16], out[17], out[18], out[19]]);
    assert_eq!(payload_len, 7);
    let status = i32::from_le_bytes([out[20], out[21], out[22], out[23]]);
    assert_eq!(status, 0);
    assert_eq!(&out[24..27], &[1, 2, 3]);
    assert_eq!(out.len(), HDR_LEN + 4 + 3);
}
