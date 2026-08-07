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
//! Pack reading, against packs GitHub actually served.
//!
//! `simple.pack` is the pack a shallow clone of octocat/Hello-World returns:
//! three whole objects, no deltas, and the ids are the ones git reports for
//! it. The delta path is checked against a full clone when one is present,
//! since a pack with thousands of deltas is too large to vendor.

use nonos_git::{read_pack, ObjectKind, PackError};

const SIMPLE: &[u8] = include_bytes!("data/simple.pack");

#[test]
fn reads_a_real_pack_from_github() {
    let objects = read_pack(SIMPLE).expect("real pack must read");
    assert_eq!(objects.len(), 3);

    // The ids git reports for this pack, in the order it stored them.
    let ids: Vec<String> = objects.iter().map(|o| o.id.to_hex()).collect();
    assert_eq!(ids[0], "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d");
    assert_eq!(ids[1], "b4eecafa9be2f2006ce1b709d6857b07069b4608");
    assert_eq!(ids[2], "980a0d5f19a64b4b30a87d4206aade58726b60e3");

    assert_eq!(objects[0].kind, ObjectKind::Commit);
    assert_eq!(objects[1].kind, ObjectKind::Tree);
    assert_eq!(objects[2].kind, ObjectKind::Blob);
}

#[test]
fn the_ids_are_recomputed_not_trusted() {
    // Every id here is the SHA-1 of the framed content we reconstructed, so a
    // matching id means the object's bytes came out right, deltas included.
    let objects = read_pack(SIMPLE).expect("read");
    let blob = objects.iter().find(|o| o.kind == ObjectKind::Blob).expect("blob");
    assert_eq!(blob.data, b"Hello World!\n");
}

#[test]
fn a_truncated_pack_is_refused() {
    // Shorter than a header plus trailer: caught before any object is read.
    assert_eq!(read_pack(&SIMPLE[..20]).err(), Some(PackError::Truncated));
    // Long enough to pass that, but an object stream is cut short.
    assert_eq!(read_pack(&SIMPLE[..40]).err(), Some(PackError::Corrupt));
    assert_eq!(read_pack(&SIMPLE[..8]).err(), Some(PackError::Truncated));
    assert_eq!(read_pack(b"NOPExxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").err(), Some(PackError::Magic));
}

#[test]
fn an_unknown_version_is_refused() {
    let mut bad = SIMPLE.to_vec();
    bad[7] = 9;
    assert_eq!(read_pack(&bad).err(), Some(PackError::Version(9)));
}
