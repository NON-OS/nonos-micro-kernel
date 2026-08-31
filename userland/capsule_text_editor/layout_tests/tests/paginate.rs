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
use capsule_text_editor_layout_tests::doc::counts::{char_count, word_count};
use capsule_text_editor_layout_tests::doc::document::Doc;
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::measure::FixedMeasurer;
use capsule_text_editor_layout_tests::doc::page::PageMetrics;
use capsule_text_editor_layout_tests::doc::paginate::paginate;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

fn metrics() -> PageMetrics {
    PageMetrics { width: 816.0, height: 200.0, margin: 48.0 }
}

fn doc_of(n: usize) -> Doc {
    let mut d = Doc::new();
    for _ in 0..n {
        d.blocks.push(Block::plain(BlockKind::Paragraph, "line", RunStyle::body()));
    }
    d
}

#[test]
fn an_empty_document_is_one_page() {
    let pages = paginate(&Doc::new(), &metrics(), &FixedMeasurer);
    assert_eq!(pages.len(), 1);
    assert!(pages[0].lines.is_empty());
}

#[test]
fn content_shorter_than_a_page_stays_on_one_page() {
    let pages = paginate(&doc_of(3), &metrics(), &FixedMeasurer);
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].lines.len(), 3);
}

#[test]
fn content_taller_than_a_page_spills_onto_the_next() {
    let pages = paginate(&doc_of(40), &metrics(), &FixedMeasurer);
    assert!(pages.len() > 1, "40 lines must not fit in a 104px content box");
}

#[test]
fn no_line_is_dropped_or_duplicated_across_pages() {
    let d = doc_of(40);
    let pages = paginate(&d, &metrics(), &FixedMeasurer);
    let total: usize = pages.iter().map(|p| p.lines.len()).sum();
    assert_eq!(total, 40);
    let mut seen: Vec<usize> = pages
        .iter()
        .flat_map(|p| p.lines.iter().map(|l| l.block))
        .collect();
    seen.sort_unstable();
    seen.dedup();
    assert_eq!(seen.len(), 40, "every block appears exactly once");
}

#[test]
fn lines_stack_downward_within_a_page() {
    let pages = paginate(&doc_of(3), &metrics(), &FixedMeasurer);
    let l = &pages[0].lines;
    assert_eq!(l[0].y, 0.0);
    assert!(l[1].y > l[0].y);
    assert!(l[2].y > l[1].y);
    assert!(l[2].y + l[2].height <= metrics().height - 2.0 * metrics().margin);
}

#[test]
fn word_and_char_counts_match_the_text() {
    let mut d = Doc::new();
    d.blocks.push(Block::plain(BlockKind::Heading(1), "Project Proposal", RunStyle::heading(1)));
    d.blocks.push(Block::plain(BlockKind::Paragraph, "one two  three", RunStyle::body()));
    assert_eq!(word_count(&d), 5);
    assert_eq!(char_count(&d), 30);
}
