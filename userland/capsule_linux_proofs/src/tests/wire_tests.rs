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


//! The Wayland wire format, in both directions.

use crate::args::Args;
use crate::wire::{next, HEADER};

/// A real wl_display.get_registry: object 1, opcode 1, one new_id of 2.
fn get_registry() -> [u8; 12] {
    let mut m = [0u8; 12];
    m[0..4].copy_from_slice(&1u32.to_le_bytes());
    m[4..6].copy_from_slice(&1u16.to_le_bytes());
    m[6..8].copy_from_slice(&12u16.to_le_bytes());
    m[8..12].copy_from_slice(&2u32.to_le_bytes());
    m
}

#[test]
fn a_message_reports_its_object_opcode_and_length() {
    let buf = get_registry();
    let (msg, size) = next(&buf).expect("a whole message parses");
    assert_eq!((msg.object, msg.opcode, size), (1, 1, 12));
    assert_eq!(Args::new(msg.args).u32(), Some(2));
}

#[test]
fn a_message_that_has_not_all_arrived_is_not_parsed() {
    let buf = get_registry();
    for cut in 0..buf.len() {
        assert!(next(&buf[..cut]).is_none(), "{cut} bytes must not parse");
    }
}

#[test]
fn a_size_smaller_than_the_header_is_refused() {
    let mut buf = get_registry();
    buf[6..8].copy_from_slice(&4u16.to_le_bytes());
    assert!(next(&buf).is_none());
}

#[test]
fn two_messages_back_to_back_are_taken_in_order() {
    let mut buf = Vec::new();
    buf.extend_from_slice(&get_registry());
    buf.extend_from_slice(&get_registry());
    let (_, first) = next(&buf).expect("first");
    let (second, _) = next(&buf[first..]).expect("second");
    assert_eq!(second.object, 1);
    assert_eq!(first, HEADER + 4);
}
