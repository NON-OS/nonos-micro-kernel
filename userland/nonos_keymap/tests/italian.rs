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
//! The characters an Italian keyboard has to be able to type.
//!
//! Every expectation here is what the key prints on a physical IT board.

use nonos_keymap::{iso, resolve, Layout};

fn plain(base: char) -> char {
    char::from_u32(resolve(base as u32, false, false, false, Layout::It)).expect("codepoint")
}

fn alt(base: char, shift: bool) -> char {
    char::from_u32(resolve(base as u32, shift, false, true, Layout::It)).expect("codepoint")
}

#[test]
fn angle_brackets_are_on_the_iso_key() {
    assert_eq!(char::from_u32(iso(Layout::It, false, false)), Some('<'));
    assert_eq!(char::from_u32(iso(Layout::It, true, false)), Some('>'));
}

#[test]
fn square_brackets_and_braces_are_on_altgr() {
    // The two keys right of P, by their US positions.
    assert_eq!(alt('[', false), '[');
    assert_eq!(alt(']', false), ']');
    assert_eq!(alt('[', true), '{');
    assert_eq!(alt(']', true), '}');
}

#[test]
fn at_and_hash_are_on_altgr() {
    assert_eq!(alt(';', false), '@');
    assert_eq!(alt('\'', false), '#');
}

#[test]
fn altgr_leaves_the_ordinary_level_alone() {
    // Without AltGr those keys still print the accented vowels.
    assert_eq!(plain('['), '\u{00E8}');
    assert_eq!(plain(';'), '\u{00F2}');
    // A key with nothing on its third level falls back rather than blanking.
    assert_eq!(alt('a', false), 'a');
}
