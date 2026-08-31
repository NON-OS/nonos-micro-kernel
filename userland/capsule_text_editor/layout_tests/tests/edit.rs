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

use capsule_text_editor_layout_tests::doc::block::{Block, Run};
use capsule_text_editor_layout_tests::doc::edit::{delete, insert};
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

fn two_run_block() -> Block {
    let mut b = Block::plain(BlockKind::Paragraph, "abcdef", RunStyle::body());
    let mut bold = RunStyle::body();
    bold.bold = true;
    b.runs[0].len = 3;
    b.runs.push(Run { len: 3, style: bold });
    b
}

#[test]
fn insert_into_the_first_run_keeps_coverage() {
    let mut b = two_run_block();
    insert(&mut b, 1, "XY");
    assert_eq!(b.as_str(), "aXYbcdef");
    assert!(b.covered());
    assert_eq!(b.runs[0].len, 5);
    assert_eq!(b.runs[1].len, 3);
}

#[test]
fn insert_inherits_the_style_at_the_offset() {
    let mut b = two_run_block();
    insert(&mut b, 4, "Z");
    assert!(b.covered());
    assert!(b.style_at(4).bold);
}

#[test]
fn insert_at_the_end_extends_the_last_run() {
    let mut b = two_run_block();
    insert(&mut b, 6, "!");
    assert_eq!(b.as_str(), "abcdef!");
    assert!(b.covered());
    assert_eq!(b.runs[1].len, 4);
}

#[test]
fn delete_spanning_two_runs_trims_both() {
    let mut b = two_run_block();
    delete(&mut b, 2, 2);
    assert_eq!(b.as_str(), "abef");
    assert!(b.covered());
    assert_eq!(b.runs[0].len, 2);
    assert_eq!(b.runs[1].len, 2);
}

#[test]
fn delete_that_empties_a_run_drops_it() {
    let mut b = two_run_block();
    delete(&mut b, 0, 3);
    assert_eq!(b.as_str(), "def");
    assert!(b.covered());
    assert_eq!(b.runs.len(), 1);
    assert!(b.runs[0].style.bold);
}

#[test]
fn deleting_everything_leaves_one_empty_run() {
    let mut b = two_run_block();
    delete(&mut b, 0, 6);
    assert_eq!(b.as_str(), "");
    assert!(b.covered());
    assert_eq!(b.runs.len(), 1);
    assert_eq!(b.runs[0].len, 0);
}

#[test]
fn delete_past_the_end_is_clamped() {
    let mut b = two_run_block();
    delete(&mut b, 4, 999);
    assert_eq!(b.as_str(), "abcd");
    assert!(b.covered());
}
