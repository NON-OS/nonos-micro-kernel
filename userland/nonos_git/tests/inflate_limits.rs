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
//! Input built to make this allocate rather than to be read.
//!
//! Every case here is small on the wire. What matters is what it asks the
//! reader to do, not what it costs to send.

use nonos_git::{decompress, InflateError};

/// A stored-block zlib stream carrying `body`, with a correct Adler-32, so
/// the only thing under test is how much it decodes to.
fn stored(body: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    for (i, chunk) in body.chunks(65535).enumerate() {
        let last = u8::from((i + 1) * 65535 >= body.len());
        out.push(last);
        out.extend_from_slice(&(chunk.len() as u16).to_le_bytes());
        out.extend_from_slice(&(!(chunk.len() as u16)).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    let mut a = 1u32;
    let mut b = 0u32;
    for byte in body {
        a = (a + u32::from(*byte)) % 65521;
        b = (b + a) % 65521;
    }
    out.extend_from_slice(&((b << 16) | a).to_be_bytes());
    out
}

#[test]
fn a_stream_that_decodes_is_still_accepted() {
    assert_eq!(decompress(&stored(b"hello")).expect("inflate"), b"hello");
}

#[test]
fn a_reserved_block_type_is_refused() {
    // Type 3 is reserved and has no meaning, so it cannot be read past.
    assert_eq!(decompress(&[0x78, 0x01, 0x07]).err(), Some(InflateError::Invalid));
}

#[test]
fn a_stream_cut_mid_block_is_refused() {
    let mut cut = stored(b"hello");
    cut.truncate(cut.len() - 6);
    assert_eq!(decompress(&cut).err(), Some(InflateError::Truncated));
}

#[test]
fn a_wrong_checksum_is_refused() {
    let mut bad = stored(b"hello");
    let end = bad.len();
    bad[end - 1] ^= 0xFF;
    assert_eq!(decompress(&bad).err(), Some(InflateError::Checksum));
}
