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
use capsule_text_editor_layout_tests::doc::document::Doc;
use capsule_text_editor_layout_tests::doc::hit::{caret_at, caret_rect, line_for};
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::measure::FixedMeasurer;
use capsule_text_editor_layout_tests::doc::page::PageMetrics;
use capsule_text_editor_layout_tests::doc::paginate::paginate;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

fn fixture() -> (Doc, PageMetrics) {
    let mut d = Doc::new();
    d.blocks.push(Block::plain(BlockKind::Heading(1), "Project Proposal", RunStyle::heading(1)));
    d.blocks.push(Block::plain(BlockKind::Paragraph, "hello world", RunStyle::body()));
    (d, PageMetrics { width: 816.0, height: 1056.0, margin: 48.0 })
}

#[test]
fn caret_rect_and_caret_at_round_trip_at_every_offset() {
    let (d, pm) = fixture();
    let pages = paginate(&d, &pm, &FixedMeasurer);
    let p = &pages[0];
    for (bi, b) in d.blocks.iter().enumerate() {
        for off in 0..=b.text.len() {
            if !b.as_str().is_char_boundary(off) {
                continue;
            }
            let (x, y, h) = caret_rect(p, &d, bi, off, &FixedMeasurer)
                .unwrap_or_else(|| panic!("no rect for block {bi} offset {off}"));
            let back = caret_at(p, &d, x + 0.1, y + h * 0.5, &FixedMeasurer);
            assert_eq!(back, (bi, off), "round trip failed at block {bi} offset {off}");
        }
    }
}

#[test]
fn caret_x_advances_monotonically_along_a_line() {
    let (d, pm) = fixture();
    let pages = paginate(&d, &pm, &FixedMeasurer);
    let mut last = -1.0f32;
    for off in 0..=11 {
        let (x, _, _) = caret_rect(&pages[0], &d, 1, off, &FixedMeasurer).unwrap();
        assert!(x > last, "caret x must increase: {x} after {last}");
        last = x;
    }
}

#[test]
fn a_click_left_of_the_text_lands_at_offset_zero() {
    let (d, pm) = fixture();
    let pages = paginate(&d, &pm, &FixedMeasurer);
    let (_, y, h) = caret_rect(&pages[0], &d, 1, 0, &FixedMeasurer).unwrap();
    assert_eq!(caret_at(&pages[0], &d, -50.0, y + h * 0.5, &FixedMeasurer), (1, 0));
}

#[test]
fn a_click_right_of_the_text_lands_at_the_end_of_the_line() {
    let (d, pm) = fixture();
    let pages = paginate(&d, &pm, &FixedMeasurer);
    let (_, y, h) = caret_rect(&pages[0], &d, 1, 0, &FixedMeasurer).unwrap();
    assert_eq!(caret_at(&pages[0], &d, 5000.0, y + h * 0.5, &FixedMeasurer), (1, 11));
}

#[test]
fn a_click_below_the_last_line_falls_back_to_the_last_line() {
    let (d, pm) = fixture();
    let pages = paginate(&d, &pm, &FixedMeasurer);
    assert_eq!(caret_at(&pages[0], &d, 5000.0, 99999.0, &FixedMeasurer), (1, 11));
    assert_eq!(caret_at(&pages[0], &d, -50.0, 99999.0, &FixedMeasurer), (1, 0));
}

#[test]
fn a_click_above_the_first_line_falls_back_to_the_first_line() {
    let (d, pm) = fixture();
    let pages = paginate(&d, &pm, &FixedMeasurer);
    assert_eq!(caret_at(&pages[0], &d, -50.0, -99999.0, &FixedMeasurer), (0, 0));
    assert_eq!(caret_at(&pages[0], &d, 5000.0, -99999.0, &FixedMeasurer), (0, 16));
}

#[test]
fn a_caret_late_in_a_long_block_belongs_to_a_later_page() {
    let mut text = String::from("word");
    for _ in 0..300 {
        text.push_str(" word");
    }
    let mut d = Doc::new();
    d.blocks.push(Block::plain(BlockKind::Paragraph, &text, RunStyle::body()));
    let pm = PageMetrics { width: 816.0, height: 200.0, margin: 48.0 };
    let pages = paginate(&d, &pm, &FixedMeasurer);
    assert!(pages.len() > 1, "the block must span several pages");
    let off = d.blocks[0].text.len();
    assert!(line_for(&pages[0], 0, off).is_none(), "page 0 must not claim the last offset");
    let page = pages.iter().position(|p| line_for(p, 0, off).is_some());
    assert_eq!(page, Some(pages.len() - 1));
}
