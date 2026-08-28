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

use capsule_text_editor_layout_tests::doc::block::Block;
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::restyle::set_style;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

fn para() -> Block {
    Block::plain(BlockKind::Paragraph, "abcdef", RunStyle::body())
}

#[test]
fn styling_the_middle_splits_into_three_runs() {
    let mut b = para();
    set_style(&mut b, 2, 2, &|s| s.bold = true);
    assert!(b.covered());
    assert_eq!(b.runs.len(), 3);
    assert!(!b.style_at(1).bold);
    assert!(b.style_at(2).bold);
    assert!(b.style_at(3).bold);
    assert!(!b.style_at(4).bold);
}

#[test]
fn styling_the_whole_block_stays_one_run() {
    let mut b = para();
    set_style(&mut b, 0, 6, &|s| s.italic = true);
    assert!(b.covered());
    assert_eq!(b.runs.len(), 1);
    assert!(b.style_at(0).italic);
}

#[test]
fn toggling_back_merges_the_runs_again() {
    let mut b = para();
    set_style(&mut b, 2, 2, &|s| s.bold = true);
    assert_eq!(b.runs.len(), 3);
    set_style(&mut b, 2, 2, &|s| s.bold = false);
    assert!(b.covered());
    assert_eq!(b.runs.len(), 1);
}

#[test]
fn a_zero_length_range_changes_nothing() {
    let mut b = para();
    set_style(&mut b, 3, 0, &|s| s.bold = true);
    assert_eq!(b.runs.len(), 1);
    assert!(!b.style_at(3).bold);
}

#[test]
fn size_and_colour_are_settable() {
    let mut b = para();
    set_style(&mut b, 0, 3, &|s| {
        s.size_px = 24.0;
        s.color = 0xFF17BED9;
    });
    assert!(b.covered());
    assert_eq!(b.style_at(0).size_px, 24.0);
    assert_eq!(b.style_at(0).color, 0xFF17BED9);
    assert_eq!(b.style_at(4).size_px, 16.0);
}
