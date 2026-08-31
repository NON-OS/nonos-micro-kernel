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
use capsule_text_editor_layout_tests::doc::style::RunStyle;

#[test]
fn plain_block_is_one_run_covering_the_text() {
    let b = Block::plain(BlockKind::Paragraph, "hello", RunStyle::body());
    assert_eq!(b.as_str(), "hello");
    assert_eq!(b.runs.len(), 1);
    assert_eq!(b.runs[0].len, 5);
    assert!(b.covered());
}

#[test]
fn style_at_reads_the_run_containing_the_offset() {
    let mut b = Block::plain(BlockKind::Paragraph, "abcdef", RunStyle::body());
    let mut bold = RunStyle::body();
    bold.bold = true;
    b.runs[0].len = 3;
    b.runs.push(capsule_text_editor_layout_tests::doc::block::Run { len: 3, style: bold });
    assert!(b.covered());
    assert!(!b.style_at(0).bold);
    assert!(!b.style_at(2).bold);
    assert!(b.style_at(3).bold);
    assert!(b.style_at(5).bold);
}

#[test]
fn style_at_past_the_end_returns_the_last_run() {
    let b = Block::plain(BlockKind::Heading(2), "hi", RunStyle::heading(2));
    assert!(b.style_at(99).bold);
}

#[test]
fn covered_is_false_when_runs_disagree_with_text() {
    let mut b = Block::plain(BlockKind::Paragraph, "hello", RunStyle::body());
    b.runs[0].len = 4;
    assert!(!b.covered());
}

#[test]
fn multibyte_text_is_measured_in_bytes() {
    let b = Block::plain(BlockKind::Paragraph, "héllo", RunStyle::body());
    assert_eq!(b.runs[0].len, 6);
    assert!(b.covered());
}
