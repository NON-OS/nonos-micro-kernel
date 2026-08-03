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
//! Pack reading, against a pack GitHub actually served.
//!
//! `simple.pack` is what a shallow clone of octocat/Hello-World returns: three
//! whole objects, ids as git reports them. Deltas are covered separately.

mod reseal;

use nonos_git::{read_pack, ObjectKind, PackError};
use reseal::reseal;

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
fn the_trailer_is_checked_first() {
    // Any cut leaves the last twenty bytes covering the wrong data, so a
    // truncated pack fails on its checksum rather than deeper in.
    assert_eq!(read_pack(&SIMPLE[..40]).err(), Some(PackError::Checksum));
    assert_eq!(read_pack(&SIMPLE[..20]).err(), Some(PackError::Checksum));
    // Too short to hold a trailer at all.
    assert_eq!(read_pack(&SIMPLE[..8]).err(), Some(PackError::Truncated));
}

#[test]
fn the_magic_and_version_are_still_checked() {
    // Both come after the checksum, so the trailer is made to match first.
    let mut bad = SIMPLE.to_vec();
    bad[7] = 9;
    reseal(&mut bad);
    assert_eq!(read_pack(&bad).err(), Some(PackError::Version(9)));

    let mut wrong_magic = SIMPLE.to_vec();
    wrong_magic[..4].copy_from_slice(b"NOPE");
    reseal(&mut wrong_magic);
    assert_eq!(read_pack(&wrong_magic).err(), Some(PackError::Magic));
}
