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

use crate::fm_logic::open_with_table::handlers_for;
use crate::fm_logic::undo::{Op, UndoStack};

#[test]
fn open_with_maps_known_extensions() {
    assert_eq!(handlers_for("/a/song.flac"), &["app.audio_player"]);
    assert_eq!(handlers_for("/a/notes.txt"), &["app.text_editor"]);
}

#[test]
fn open_with_is_case_insensitive() {
    assert_eq!(handlers_for("/a/SONG.FLAC"), &["app.audio_player"]);
}

#[test]
fn open_with_returns_empty_for_an_unclaimed_type() {
    assert!(handlers_for("/a/blob.zzz").is_empty());
    assert!(handlers_for("/a/noext").is_empty());
}

#[test]
fn undo_pops_in_reverse_order() {
    let mut u = UndoStack::default();
    u.push(Op::Unlink { path: alloc::string::String::from("/a") });
    u.push(Op::Rmdir { path: alloc::string::String::from("/d") });
    assert!(matches!(u.pop(), Some(Op::Rmdir { .. })));
    assert!(matches!(u.pop(), Some(Op::Unlink { .. })));
    assert!(u.pop().is_none());
}

#[test]
fn undo_is_bounded_at_16() {
    let mut u = UndoStack::default();
    for i in 0..20 {
        u.push(Op::Unlink { path: alloc::format!("/f{i}") });
    }
    assert_eq!(u.len(), 16);
    assert!(matches!(u.pop(), Some(Op::Unlink { ref path }) if path == "/f19"));
}

#[test]
fn undo_clear_empties_the_stack() {
    let mut u = UndoStack::default();
    u.push(Op::Unlink { path: alloc::string::String::from("/a") });
    u.clear();
    assert_eq!(u.len(), 0);
}
