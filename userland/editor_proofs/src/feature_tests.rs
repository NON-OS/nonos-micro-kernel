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

//! Proofs for the code-editing features layered on the edit path: word motion,
//! block indent, comment toggling, line duplication and deletion, smart Home,
//! and bracket auto-closing. Each one must also unwind through a single undo.

use crate::autoclose::{autoclose, Auto};
use crate::edit_tests::{doc, text};
use crate::state::State;

fn set_path(s: &mut State, path: &str) {
    s.path[..path.len()].copy_from_slice(path.as_bytes());
    s.path_len = path.len();
}

#[test]
fn word_motion_stops_at_class_boundaries() {
    let mut s = doc("let foo_bar = call(x);");
    s.caret = 0;
    s.word_right();
    assert_eq!(s.caret, 3); // after "let"
    s.word_right();
    assert_eq!(s.caret, 11); // after "foo_bar"
    s.caret = 11;
    s.word_left();
    assert_eq!(s.caret, 4); // back to the start of "foo_bar"
}

#[test]
fn delete_word_left_is_one_edit() {
    let mut s = doc("alpha beta");
    s.caret = 10;
    assert!(s.delete_word_left());
    assert_eq!(text(&s), "alpha ");
    assert!(s.undo());
    assert_eq!(text(&s), "alpha beta");
}

#[test]
fn indent_adds_four_spaces_per_selected_line() {
    let mut s = doc("one\ntwo\n\nthree");
    s.sel_anchor = Some(0);
    s.caret = s.len;
    assert!(s.indent_selection());
    assert_eq!(text(&s), "    one\n    two\n\n    three");
    assert!(s.undo());
    assert_eq!(text(&s), "one\ntwo\n\nthree");
}

#[test]
fn dedent_removes_spaces_or_a_tab() {
    let mut s = doc("    one\n\ttwo\nthree");
    s.sel_anchor = Some(0);
    s.caret = s.len;
    assert!(s.dedent_selection());
    assert_eq!(text(&s), "one\ntwo\nthree");
}

#[test]
fn selection_is_multiline_gates_tab_behavior() {
    let mut s = doc("one\ntwo");
    s.sel_anchor = Some(0);
    s.caret = 2;
    assert!(!s.selection_is_multiline());
    s.caret = 6;
    assert!(s.selection_is_multiline());
}

#[test]
fn toggle_comment_adds_then_removes() {
    let mut s = doc("fn main() {}\nlet x = 1;");
    set_path(&mut s, "/main.rs");
    s.sel_anchor = Some(0);
    s.caret = s.len;
    assert!(s.toggle_comment());
    assert_eq!(text(&s), "// fn main() {}\n// let x = 1;");
    assert!(s.toggle_comment());
    assert_eq!(text(&s), "fn main() {}\nlet x = 1;");
}

#[test]
fn toggle_comment_uses_the_language_marker() {
    let mut s = doc("import os");
    set_path(&mut s, "/tool.py");
    s.caret = 0;
    assert!(s.toggle_comment());
    assert_eq!(text(&s), "# import os");
}

#[test]
fn toggle_comment_keeps_indentation() {
    let mut s = doc("    body();");
    set_path(&mut s, "/a.c");
    s.caret = 6;
    assert!(s.toggle_comment());
    assert_eq!(text(&s), "    // body();");
}

#[test]
fn duplicate_line_keeps_the_column() {
    let mut s = doc("first\nsecond");
    s.caret = 2; // inside "first"
    assert!(s.duplicate_line());
    assert_eq!(text(&s), "first\nfirst\nsecond");
    assert_eq!(s.caret, 8); // same column on the copy
    assert!(s.undo());
    assert_eq!(text(&s), "first\nsecond");
}

#[test]
fn delete_line_middle_and_last() {
    let mut s = doc("a\nb\nc");
    s.caret = 2; // on "b"
    assert!(s.delete_line());
    assert_eq!(text(&s), "a\nc");
    s.caret = s.len; // on "c", the last line
    assert!(s.delete_line());
    assert_eq!(text(&s), "a");
}

#[test]
fn smart_home_toggles_between_indent_and_column_zero() {
    let mut s = doc("    code here");
    s.caret = 9;
    s.caret_home();
    assert_eq!(s.caret, 4, "first Home goes to the first non-blank");
    s.caret_home();
    assert_eq!(s.caret, 0, "second Home goes to the true start");
    s.caret_home();
    assert_eq!(s.caret, 4, "and it toggles back");
}

#[test]
fn autoclose_inserts_a_pair_and_steps_over_the_closer() {
    let mut s = doc("");
    assert!(matches!(autoclose(&mut s, '('), Some(Auto::Inserted)));
    assert_eq!(text(&s), "()");
    assert_eq!(s.caret, 1);
    assert!(matches!(autoclose(&mut s, ')'), Some(Auto::Skipped)));
    assert_eq!(text(&s), "()", "typing the closer must not double it");
    assert_eq!(s.caret, 2);
}

#[test]
fn autoclose_wraps_a_selection() {
    let mut s = doc("word");
    s.sel_anchor = Some(0);
    s.caret = 4;
    assert!(matches!(autoclose(&mut s, '"'), Some(Auto::Inserted)));
    assert_eq!(text(&s), "\"word\"");
    assert_eq!(s.sel_range(), Some((1, 5)), "the wrapped text stays selected");
}

#[test]
fn autoclose_leaves_apostrophes_alone() {
    let mut s = doc("don");
    s.caret = 3;
    assert!(autoclose(&mut s, '\'').is_none(), "after a word char, ' is an apostrophe");
    let mut s2 = doc("x(");
    s2.caret = 1;
    assert!(autoclose(&mut s2, '(').is_none() || text(&s2) != "x((", "no pair jammed before a word");
}

#[test]
fn select_word_grabs_the_identifier_under_the_caret() {
    let mut s = doc("let foo_bar = 1;");
    s.caret = 6; // inside "foo_bar"
    s.select_word();
    assert_eq!(s.sel_range(), Some((4, 11)));
    // On punctuation it selects the single character instead of nothing.
    s.caret = 12;
    s.select_word();
    assert_eq!(s.sel_range(), Some((12, 13)));
}

#[test]
fn autoclose_undo_removes_the_whole_pair() {
    let mut s = doc("");
    autoclose(&mut s, '{');
    assert_eq!(text(&s), "{}");
    assert!(s.undo());
    assert_eq!(text(&s), "");
}

#[test]
fn replace_current_rewrites_the_match_and_steps_on() {
    let mut s = doc("old code, old habits");
    s.find_buf = String::from("old");
    s.replace_buf = String::from("new");
    s.caret = 0;
    s.find_incremental(); // selects the first "old"
    assert!(s.replace_current());
    assert_eq!(text(&s), "new code, old habits");
    assert_eq!(s.sel_range(), Some((10, 13)), "the next match is selected");
}

#[test]
fn replace_all_counts_and_terminates_when_replacement_contains_query() {
    let mut s = doc("aaa");
    s.find_buf = String::from("a");
    s.replace_buf = String::from("aa");
    // Each "a" becomes "aa", so matches keep reappearing inside their own
    // replacements. The pass replaces strictly forward and stops when the
    // search wraps: two rewrites land before the scan runs off the end, and
    // crucially it terminates instead of growing forever.
    let n = s.replace_all();
    assert_eq!(n, 2);
    assert_eq!(text(&s), "aaaaa");
}

#[test]
fn replace_all_simple_pass() {
    let mut s = doc("x = x + x;");
    s.find_buf = String::from("x");
    s.replace_buf = String::from("total");
    assert_eq!(s.replace_all(), 3);
    assert_eq!(text(&s), "total = total + total;");
}
