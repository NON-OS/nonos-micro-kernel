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

use capsule_text_editor_layout_tests::doc::export::zip::{crc32, Zip};

fn le16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn le32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

#[test]
fn crc32_known_vectors() {
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    assert_eq!(crc32(b""), 0);
    assert_eq!(crc32(b"a"), 0xE8B7_BE43);
}

#[test]
fn local_header_sits_at_offset_zero() {
    let mut z = Zip::new();
    z.add("a.txt", b"hello");
    let out = z.finish();
    assert_eq!(&out[0..4], b"PK\x03\x04");
    assert_eq!(le16(&out, 8), 0);
    assert_eq!(le32(&out, 14), crc32(b"hello"));
    assert_eq!(le32(&out, 18), 5);
    assert_eq!(le32(&out, 22), 5);
    assert_eq!(le16(&out, 26), 5);
    assert_eq!(le16(&out, 28), 0);
    assert_eq!(&out[30..35], b"a.txt");
    assert_eq!(&out[35..40], b"hello");
}

#[test]
fn eocd_offsets_are_consistent() {
    let mut z = Zip::new();
    z.add("one", b"first");
    z.add("two", b"second-entry");
    let out = z.finish();
    let e = out.len() - 22;
    assert_eq!(&out[e..e + 4], b"PK\x05\x06");
    assert_eq!(le16(&out, e + 8), 2);
    assert_eq!(le16(&out, e + 10), 2);
    let cd = le32(&out, e + 16) as usize;
    assert_eq!(cd + le32(&out, e + 12) as usize, e);
    assert_eq!(&out[cd..cd + 4], b"PK\x01\x02");
    assert_eq!(le32(&out, cd + 42), 0);
    let second = cd + 46 + 3;
    assert_eq!(&out[second..second + 4], b"PK\x01\x02");
    let local2 = le32(&out, second + 42) as usize;
    assert_eq!(&out[local2..local2 + 4], b"PK\x03\x04");
    assert_eq!(&out[local2 + 30..local2 + 33], b"two");
}
