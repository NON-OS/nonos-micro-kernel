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
//! Deltas built to make the reader allocate.

use nonos_git::{read_pack, PackError, Sha1};

/// Wrap `entries` in a pack header and a correct trailer, so the delta is the
/// only thing the reader can object to.
fn pack(count: u32, entries: &[u8]) -> Vec<u8> {
    let mut out = Vec::from(&b"PACK"[..]);
    out.extend_from_slice(&2u32.to_be_bytes());
    out.extend_from_slice(&count.to_be_bytes());
    out.extend_from_slice(entries);
    let sha = Sha1::digest(&out);
    out.extend_from_slice(&sha);
    out
}

/// An ofs-delta entry header pointing `back` bytes, with `body` as its
/// compressed payload.
fn ofs_delta(back: u8, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    // Type 6 in bits 4 to 6, size in the low four. Size is not read for this.
    out.push(0x60 | 0x01);
    out.push(back);
    out.extend_from_slice(body);
    out
}

#[test]
fn a_delta_claiming_an_enormous_target_is_refused() {
    // Ten continuation bytes of 0x7F: a target of nearly 2^70, asserted by
    // eleven bytes on the wire. Reserving for it is what this must not do.
    let mut header = vec![0x00];
    header.extend_from_slice(&[0xFF; 10]);
    header.push(0x01);
    let entry = ofs_delta(0x01, &header);
    let err = read_pack(&pack(1, &entry)).err();
    assert!(err.is_some(), "a delta of that size must not be reconstructed");
}

#[test]
fn a_delta_whose_base_is_absent_is_refused() {
    // Nothing precedes it, so there is nothing to count back to.
    let entry =
        ofs_delta(0x7F, &[0x78, 0x01, 0x01, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01]);
    assert!(matches!(
        read_pack(&pack(1, &entry)).err(),
        Some(PackError::MissingBase) | Some(PackError::Corrupt)
    ));
}
