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

use nonos_git::{compress, decompress, InflateError};

fn unhex(s: &str) -> Vec<u8> {
    let b = s.as_bytes();
    (0..b.len() / 2)
        .map(|i| {
            let hi = (b[i * 2] as char).to_digit(16).unwrap() as u8;
            let lo = (b[i * 2 + 1] as char).to_digit(16).unwrap() as u8;
            (hi << 4) | lo
        })
        .collect()
}

#[test]
fn inflates_a_real_git_object() {
    // The exact bytes of .git/objects/ce/013625... for `printf 'hello\n'`.
    // Git used Huffman coding, so this exercises the dynamic path.
    let on_disk = unhex("78014bcac94f523063c848cdc9c9e702001dc50414");
    assert_eq!(decompress(&on_disk).expect("real git object"), b"blob 6\0hello\n");
}

#[test]
fn round_trips_arbitrary_data() {
    for n in [0usize, 1, 100, 65534, 65535, 65536, 200_000] {
        let data: Vec<u8> = (0..n).map(|i| (i * 31 + 7) as u8).collect();
        assert_eq!(decompress(&compress(&data)).expect("round trip"), data, "n={n}");
    }
}

#[test]
fn round_trips_repetitive_data() {
    let data = vec![0xABu8; 300_000];
    assert_eq!(decompress(&compress(&data)).unwrap(), data);
}

#[test]
fn a_corrupt_stream_is_rejected() {
    let mut z = compress(b"payload");
    let last = z.len() - 1;
    z[last] ^= 0xFF;
    assert_eq!(decompress(&z), Err(InflateError::Checksum));

    assert_eq!(decompress(&[]), Err(InflateError::Header));
    assert_eq!(decompress(&[0x00, 0x00]), Err(InflateError::Header));

    let z = compress(b"some longer payload to span the block");
    assert!(decompress(&z[..z.len() - 3]).is_err());
}
