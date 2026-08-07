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
//! Trees git would never write are refused.

use nonos_git::{parse_tree, TreeError};

/// A hand-built entry, for cases the encoder would never produce.
fn raw(mode: &[u8], name: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(mode);
    out.push(b' ');
    out.extend_from_slice(name);
    out.push(0);
    out.extend_from_slice(&[0u8; 20]);
    out
}

#[test]
fn an_out_of_order_tree_is_rejected() {
    // Accepting this would mean re-encoding gives a different id than it
    // parsed from.
    let mut out = raw(b"100644", b"b");
    out.extend_from_slice(&raw(b"100644", b"a"));
    assert_eq!(parse_tree(&out), Err(TreeError::Order));
}

#[test]
fn a_tree_round_trips_through_parse() {
    use nonos_git::{encode_tree, Mode, ObjectId, TreeEntry};
    let id = ObjectId::from_hex("ce013625030ba8dba906f756967f9e9ca394464a").unwrap();
    let mut entries = vec![
        TreeEntry { mode: Mode::File, name: String::from("z.txt"), id },
        TreeEntry { mode: Mode::Directory, name: String::from("dir"), id },
        TreeEntry { mode: Mode::Symlink, name: String::from("link"), id },
    ];
    let content = encode_tree(&mut entries);
    assert_eq!(parse_tree(&content).expect("round trip"), entries);
}
