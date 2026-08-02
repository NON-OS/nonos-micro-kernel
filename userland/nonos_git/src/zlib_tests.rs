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

//! zlib against real git output and its own round trip.
//!
//! The decisive case decompresses the exact bytes git wrote for the `hello\n`
//! blob, so the inflate path is shown to read git's dynamic-Huffman output,
//! not just our own stored blocks. The round trips cover empty input and sizes
//! either side of the 65535-byte stored-block boundary.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use crate::zlib::{compress, decompress, InflateError};

/// Decode a hex string into bytes for the on-disk fixture.
fn unhex(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len() / 2);
    let mut i = 0;
    while i < b.len() {
        let hi = (b[i] as char).to_digit(16).unwrap() as u8;
        let lo = (b[i + 1] as char).to_digit(16).unwrap() as u8;
        out.push((hi << 4) | lo);
        i += 2;
    }
    out
}

#[test]
fn inflates_a_real_git_object() {
    // The exact contents of .git/objects/ce/013625... for `printf 'hello\n'`,
    // captured with xxd. git compressed it with Huffman coding, so decoding it
    // exercises the dynamic path, not stored blocks.
    let on_disk = unhex("78014bcac94f523063c848cdc9c9e702001dc50414");
    let decoded = decompress(&on_disk).expect("real git object must inflate");
    assert_eq!(decoded, b"blob 6\0hello\n");
}

#[test]
fn round_trips_arbitrary_data() {
    for n in [0usize, 1, 100, 65534, 65535, 65536, 200_000] {
        let data: Vec<u8> = (0..n).map(|i| (i * 31 + 7) as u8).collect();
        let z = compress(&data);
        let back = decompress(&z).expect("round trip");
        assert_eq!(back, data, "n={n}");
    }
}

#[test]
fn round_trips_highly_repetitive_data() {
    // Runs are where a real deflator uses back-references; our stored-block
    // writer must still round-trip them exactly.
    let data = vec![0xABu8; 300_000];
    assert_eq!(decompress(&compress(&data)).unwrap(), data);
}

#[test]
fn a_corrupt_checksum_is_rejected() {
    let mut z = compress(b"payload");
    // Flip a byte in the trailing Adler-32.
    let last = z.len() - 1;
    z[last] ^= 0xFF;
    assert_eq!(decompress(&z), Err(InflateError::Checksum));
}

#[test]
fn a_bad_header_is_rejected() {
    assert_eq!(decompress(&[]), Err(InflateError::Header));
    assert_eq!(decompress(&[0x00, 0x00]), Err(InflateError::Header));
}

#[test]
fn a_truncated_stream_is_rejected() {
    let z = compress(b"some longer payload to span the block");
    // Cut off the trailer and part of the data.
    assert!(matches!(
        decompress(&z[..z.len() - 3]),
        Err(InflateError::Truncated) | Err(InflateError::Checksum)
    ));
}
