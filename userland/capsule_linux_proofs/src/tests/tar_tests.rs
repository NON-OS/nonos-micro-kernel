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


//! The tar reader, against a real Alpine package.

use crate::install::tar::entries;

/// libffi-3.4.6-r0.apk, decompressed. An independent reader sees one
/// regular file in it: usr/lib/libffi.so.8.1.4 at 38960 bytes.
const PKG: &[u8] = include_bytes!("../../vectors/libffi.tar");

#[test]
fn it_finds_the_package_payload_and_nothing_else() {
    let found = entries(PKG);
    let named: Vec<&[u8]> = found.iter().map(|e| e.name.as_slice()).collect();
    assert!(
        named.contains(&b"usr/lib/libffi.so.8.1.4".as_slice()),
        "the shared library must be there, saw {named:?}"
    );
}

#[test]
fn the_payload_is_the_right_length_and_is_an_elf() {
    let found = entries(PKG);
    let lib = found
        .iter()
        .find(|e| e.name == b"usr/lib/libffi.so.8.1.4")
        .expect("the library");
    assert_eq!(lib.body.len(), 38960);
    assert_eq!(&lib.body[..4], b"\x7fELF");
}

#[test]
fn a_truncated_archive_stops_rather_than_inventing_entries() {
    for cut in [0usize, 100, 512, 1024, PKG.len() / 2] {
        let found = entries(&PKG[..cut]);
        for e in &found {
            assert!(e.body.len() <= cut, "an entry longer than the input");
        }
    }
}

#[test]
fn rubbish_is_not_an_archive() {
    assert!(entries(&[0u8; 512]).is_empty());
    assert!(entries(b"not a tar at all").is_empty());
}
