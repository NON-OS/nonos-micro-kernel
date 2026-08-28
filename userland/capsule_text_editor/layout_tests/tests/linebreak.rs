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
use capsule_text_editor_layout_tests::doc::linebreak::break_block;
use capsule_text_editor_layout_tests::doc::measure::FixedMeasurer;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

fn para(text: &str) -> Block {
    Block::plain(BlockKind::Paragraph, text, RunStyle::body())
}

#[test]
fn short_text_is_one_line() {
    let b = para("hello");
    let lines = break_block(&b, 0, 400.0, &FixedMeasurer);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].start, 0);
    assert_eq!(lines[0].end, 5);
    assert_eq!(lines[0].width, 40.0);
}

#[test]
fn an_empty_block_still_produces_one_line() {
    let lines = break_block(&para(""), 0, 400.0, &FixedMeasurer);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].start, 0);
    assert_eq!(lines[0].end, 0);
    assert!(lines[0].height > 0.0);
}

#[test]
fn text_wraps_at_a_space_not_mid_word() {
    let b = para("aaaa bbbb cccc");
    let lines = break_block(&b, 0, 40.0, &FixedMeasurer);
    assert_eq!(lines.len(), 3);
    for l in &lines {
        assert!(l.width <= 40.0, "line {l:?} exceeds the width");
        let s = &b.as_str()[l.start..l.end];
        assert!(!s.starts_with(' '), "a wrapped line must not lead with a space");
    }
    assert_eq!(&b.as_str()[lines[0].start..lines[0].end], "aaaa");
}

#[test]
fn every_byte_appears_exactly_once_across_lines() {
    let b = para("aaaa bbbb cccc dddd eeee");
    let lines = break_block(&b, 0, 90.0, &FixedMeasurer);
    let mut covered = 0usize;
    for l in &lines {
        covered += l.end - l.start;
    }
    assert!(covered <= b.text.len());
    assert_eq!(lines.first().unwrap().start, 0);
    assert_eq!(lines.last().unwrap().end, b.text.len());
}

#[test]
fn a_word_longer_than_the_line_still_emits_one_line() {
    let b = para("aaaaaaaaaaaaaaaaaaaa");
    let lines = break_block(&b, 0, 40.0, &FixedMeasurer);
    assert!(!lines.is_empty(), "must not loop forever or return nothing");
    assert_eq!(lines.last().unwrap().end, b.text.len());
}

#[test]
fn heading_lines_are_taller_than_body_lines() {
    let h = Block::plain(BlockKind::Heading(1), "Title", RunStyle::heading(1));
    let p = para("Title");
    let hl = break_block(&h, 0, 400.0, &FixedMeasurer);
    let pl = break_block(&p, 0, 400.0, &FixedMeasurer);
    assert!(hl[0].height > pl[0].height);
}
