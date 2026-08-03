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
//! The third level on the other European layouts.

use nonos_keymap::{iso, resolve, Layout};

fn alt(base: char, layout: Layout) -> char {
    char::from_u32(resolve(base as u32, false, false, true, layout)).expect("codepoint")
}

#[test]
fn german_brackets_sit_on_the_number_row() {
    assert_eq!(alt('7', Layout::De), '{');
    assert_eq!(alt('8', Layout::De), '[');
    assert_eq!(alt('9', Layout::De), ']');
    assert_eq!(alt('0', Layout::De), '}');
    assert_eq!(alt('q', Layout::De), '@');
}

#[test]
fn french_brackets_sit_on_the_number_row() {
    assert_eq!(alt('4', Layout::Fr), '{');
    assert_eq!(alt('5', Layout::Fr), '[');
    assert_eq!(alt('-', Layout::Fr), ']');
    assert_eq!(alt('=', Layout::Fr), '}');
    assert_eq!(alt('0', Layout::Fr), '@');
}

#[test]
fn spanish_brackets_and_braces() {
    assert_eq!(alt('[', Layout::Es), '[');
    assert_eq!(alt(']', Layout::Es), ']');
    assert_eq!(alt('\'', Layout::Es), '{');
    assert_eq!(alt('\\', Layout::Es), '}');
    assert_eq!(alt('2', Layout::Es), '@');
}

#[test]
fn the_iso_key_follows_the_layout() {
    // A UK board prints backslash and pipe there, not the angle brackets.
    assert_eq!(char::from_u32(iso(Layout::Uk, false, false)), Some('\\'));
    assert_eq!(char::from_u32(iso(Layout::Uk, true, false)), Some('|'));
    assert_eq!(char::from_u32(iso(Layout::De, false, true)), Some('|'));
    // An ANSI board has no such key, so the press produces nothing.
    assert_eq!(iso(Layout::Us, false, false), 0);
}

#[test]
fn a_us_layout_has_no_third_level() {
    assert_eq!(alt('7', Layout::Us), '7');
    assert_eq!(alt('[', Layout::Us), '[');
}
