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

//! Proofs for the single edit path: every mutation goes through `apply_edit`,
//! so undo must return the buffer to the exact prior bytes and redo must
//! reapply them, from any interleaving of inserts, deletes, and selections.

use crate::state::State;

pub fn doc(text: &str) -> State {
    let mut s = State::new();
    s.buf[..text.len()].copy_from_slice(text.as_bytes());
    s.len = text.len();
    s
}

pub fn text(s: &State) -> &str {
    core::str::from_utf8(&s.buf[..s.len]).unwrap()
}

#[test]
fn insert_then_undo_restores_exact_bytes() {
    let mut s = doc("hello world");
    s.caret = 5;
    assert!(s.insert(b", brave"));
    assert_eq!(text(&s), "hello, brave world");
    assert!(s.undo());
    assert_eq!(text(&s), "hello world");
    assert_eq!(s.caret, 5);
}

#[test]
fn redo_reapplies_what_undo_removed() {
    let mut s = doc("abc");
    s.caret = 3;
    s.insert(b"def");
    assert!(s.undo());
    assert_eq!(text(&s), "abc");
    assert!(s.redo());
    assert_eq!(text(&s), "abcdef");
    assert_eq!(s.caret, 6);
}

#[test]
fn new_edit_clears_redo() {
    let mut s = doc("abc");
    s.caret = 3;
    s.insert(b"x");
    s.undo();
    s.insert(b"y");
    assert!(!s.redo(), "redo must be invalidated by a fresh edit");
    assert_eq!(text(&s), "abcy");
}

#[test]
fn delete_selection_is_one_undo_step() {
    let mut s = doc("keep DROP keep");
    s.sel_anchor = Some(5);
    s.caret = 10;
    assert!(s.delete_sel());
    assert_eq!(text(&s), "keep keep");
    assert!(s.undo());
    assert_eq!(text(&s), "keep DROP keep");
}

#[test]
fn backspace_and_delete_forward_are_undoable() {
    let mut s = doc("ab");
    s.caret = 1;
    assert!(s.backspace());
    assert_eq!(text(&s), "b");
    assert!(s.delete_forward());
    assert_eq!(text(&s), "");
    assert!(s.undo());
    assert!(s.undo());
    assert_eq!(text(&s), "ab");
}

#[test]
fn undo_stack_survives_a_long_interleaving() {
    let mut s = doc("");
    for i in 0..50u8 {
        s.caret = s.len;
        s.insert(&[b'a' + (i % 26)]);
        if i % 7 == 0 && s.len > 0 {
            s.caret = s.len;
            s.backspace();
        }
    }
    let final_text = String::from(text(&s));
    while s.undo() {}
    assert_eq!(text(&s), "");
    while s.redo() {}
    assert_eq!(text(&s), final_text);
}

#[test]
fn find_is_case_insensitive_and_wraps() {
    let mut s = doc("Alpha beta ALPHA gamma alpha");
    s.find_buf = String::from("alpha");
    assert_eq!(s.find_count(), 3);
    s.caret = 10;
    s.find_next(true);
    assert_eq!(s.sel_range(), Some((11, 16)));
    s.find_next(true);
    assert_eq!(s.sel_range(), Some((23, 28)));
    s.find_next(true); // wraps to the first match
    assert_eq!(s.sel_range(), Some((0, 5)));
}

#[test]
fn caret_motion_is_utf8_aware() {
    let mut s = doc("aé漢z");
    s.caret = s.len;
    s.caret_left();
    assert_eq!(&text(&s)[s.caret..], "z");
    s.caret_left();
    assert_eq!(&text(&s)[s.caret..], "漢z");
    s.caret_left();
    assert_eq!(&text(&s)[s.caret..], "é漢z");
    s.caret_right();
    assert_eq!(&text(&s)[s.caret..], "漢z");
}
