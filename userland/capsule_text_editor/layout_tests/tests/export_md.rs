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
use capsule_text_editor_layout_tests::doc::document::Doc;
use capsule_text_editor_layout_tests::doc::export::md::to_markdown;
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

fn doc_of(blocks: Vec<Block>) -> Doc {
    Doc { blocks }
}

fn styled(bold: bool, italic: bool, strike: bool) -> RunStyle {
    let mut s = RunStyle::body();
    s.bold = bold;
    s.italic = italic;
    s.strike = strike;
    s
}

#[test]
fn empty_document_exports_nothing() {
    assert_eq!(to_markdown(&Doc::new()), "");
}

#[test]
fn headings_use_one_hash_per_level_without_redundant_bold() {
    let d = doc_of(
        (1u8..=6)
            .map(|n| Block::plain(BlockKind::Heading(n), "T", RunStyle::heading(n)))
            .collect(),
    );
    let md = to_markdown(&d);
    for n in 1u8..=6 {
        let mut want = String::new();
        for _ in 0..n {
            want.push('#');
        }
        want.push_str(" T");
        assert!(md.contains(&want), "missing {:?} in {:?}", want, md);
    }
}

#[test]
fn heading_level_is_clamped_to_six() {
    let d = doc_of(vec![Block::plain(BlockKind::Heading(9), "x", RunStyle::body())]);
    assert_eq!(to_markdown(&d), "###### x\n");
}

#[test]
fn bullets_all_use_a_dash() {
    let d = doc_of(vec![
        Block::plain(BlockKind::Bullet, "one", RunStyle::body()),
        Block::plain(BlockKind::Bullet, "two", RunStyle::body()),
    ]);
    assert_eq!(to_markdown(&d), "- one\n\n- two\n");
}

#[test]
fn numbered_runs_renumber_and_restart() {
    let d = doc_of(vec![
        Block::plain(BlockKind::Numbered, "a", RunStyle::body()),
        Block::plain(BlockKind::Numbered, "b", RunStyle::body()),
        Block::plain(BlockKind::Paragraph, "p", RunStyle::body()),
        Block::plain(BlockKind::Numbered, "c", RunStyle::body()),
    ]);
    assert_eq!(to_markdown(&d), "1. a\n\n2. b\n\np\n\n1. c\n");
}

#[test]
fn paragraphs_are_bare_text_separated_by_a_blank_line() {
    let d = doc_of(vec![
        Block::plain(BlockKind::Paragraph, "first", RunStyle::body()),
        Block::plain(BlockKind::Paragraph, "second", RunStyle::body()),
    ]);
    assert_eq!(to_markdown(&d), "first\n\nsecond\n");
}
