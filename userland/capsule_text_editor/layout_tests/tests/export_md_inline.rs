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
fn markers_are_emitted_per_run_and_combine() {
    let mut b = Block::plain(BlockKind::Paragraph, "abcdefgh", RunStyle::body());
    b.runs = vec![
        Run { len: 2, style: styled(false, false, false) },
        Run { len: 2, style: styled(true, false, false) },
        Run { len: 2, style: styled(false, true, false) },
        Run { len: 2, style: styled(true, true, true) },
    ];
    assert!(b.covered());
    assert_eq!(to_markdown(&doc_of(vec![b])), "ab**cd***ef*~~***gh***~~\n");
}

#[test]
fn an_empty_run_emits_nothing() {
    let mut b = Block::plain(BlockKind::Paragraph, "hi", RunStyle::body());
    b.runs = vec![
        Run { len: 0, style: styled(true, true, true) },
        Run { len: 2, style: styled(false, false, false) },
    ];
    assert!(b.covered());
    assert_eq!(to_markdown(&doc_of(vec![b])), "hi\n");
}

#[test]
fn runs_covering_the_whole_text_reproduce_it() {
    let mut b = Block::plain(BlockKind::Paragraph, "hello world", RunStyle::body());
    b.runs = vec![
        Run { len: 6, style: RunStyle::body() },
        Run { len: 5, style: RunStyle::body() },
    ];
    assert!(b.covered());
    assert_eq!(to_markdown(&doc_of(vec![b])), "hello world\n");
}

#[test]
fn underline_has_no_markdown_and_is_dropped() {
    let mut s = RunStyle::body();
    s.underline = true;
    let d = doc_of(vec![Block::plain(BlockKind::Paragraph, "u", s)]);
    assert_eq!(to_markdown(&d), "u\n");
}

#[test]
fn significant_characters_are_escaped() {
    let d = doc_of(vec![Block::plain(
        BlockKind::Paragraph,
        "a*b_c`d[e]f\\g",
        RunStyle::body(),
    )]);
    assert_eq!(to_markdown(&d), "a\\*b\\_c\\`d\\[e\\]f\\\\g\n");
}

#[test]
fn a_leading_hash_is_escaped_only_at_line_start() {
    let d = doc_of(vec![
        Block::plain(BlockKind::Paragraph, "# not a heading", RunStyle::body()),
        Block::plain(BlockKind::Paragraph, "a # b", RunStyle::body()),
        Block::plain(BlockKind::Bullet, "# item", RunStyle::body()),
    ]);
    assert_eq!(to_markdown(&d), "\\# not a heading\n\na # b\n\n- # item\n");
}

#[test]
fn escaping_survives_inside_a_styled_run() {
    let b = Block::plain(BlockKind::Paragraph, "2*3", styled(true, false, false));
    assert_eq!(to_markdown(&doc_of(vec![b])), "**2\\*3**\n");
}

#[test]
fn multibyte_run_boundaries_do_not_panic() {
    let mut b = Block::plain(BlockKind::Paragraph, "héllo", RunStyle::body());
    b.runs = vec![
        Run { len: 3, style: styled(true, false, false) },
        Run { len: 3, style: styled(false, false, false) },
    ];
    assert!(b.covered());
    assert_eq!(to_markdown(&doc_of(vec![b])), "**hé**llo\n");
}
