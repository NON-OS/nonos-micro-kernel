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
use capsule_text_editor_layout_tests::doc::export::pdf::to_pdf;
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::measure::FixedMeasurer;
use capsule_text_editor_layout_tests::doc::page::PageMetrics;
use capsule_text_editor_layout_tests::doc::style::{Family, RunStyle};

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

fn pdf_of(doc: &Doc) -> String {
    String::from_utf8_lossy(&to_pdf(doc, &metrics(), &FixedMeasurer)).into_owned()
}

fn find(hay: &[u8], needle: &[u8]) -> usize {
    hay.windows(needle.len()).position(|w| w == needle).expect("needle not found")
}

fn tm_ys(src: &str) -> Vec<f32> {
    src.lines()
        .filter(|l| l.ends_with(" Tm"))
        .map(|l| l.split(' ').nth(5).unwrap().parse::<f32>().unwrap())
        .collect()
}

#[test]
fn output_is_a_pdf_envelope() {
    let out = to_pdf(&doc_of(2), &metrics(), &FixedMeasurer);
    assert!(out.starts_with(b"%PDF-"), "missing header");
    assert!(out.ends_with(b"%%EOF"), "missing trailer marker");
}

#[test]
fn startxref_points_at_the_xref_keyword() {
    let out = to_pdf(&doc_of(2), &metrics(), &FixedMeasurer);
    let at = find(&out, b"startxref\n") + 10;
    let digits = &out[at..out.len() - 6];
    let off: usize = core::str::from_utf8(digits).unwrap().parse().unwrap();
    assert_eq!(&out[off..off + 5], b"xref\n");
}

#[test]
fn size_counts_every_object_plus_the_free_head() {
    let src = pdf_of(&doc_of(2));
    let declared: usize =
        src.split("/Size ").nth(1).unwrap().split(' ').next().unwrap().parse().unwrap();
    let objects = src.matches(" 0 obj\n").count();
    assert_eq!(declared, objects + 1);
    assert_eq!(src.matches(" 00000 n \n").count(), objects);
}

#[test]
fn a_two_page_document_emits_two_page_objects() {
    let src = pdf_of(&doc_of(5));
    assert_eq!(src.matches("/Type /Page /Parent").count(), 2);
    assert_eq!(src.matches("/Type /Pages ").count(), 1);
    assert_eq!(src.matches("/Count 2").count(), 1);
}

#[test]
fn the_y_axis_is_flipped_so_the_first_line_sits_higher() {
    let ys = tm_ys(&pdf_of(&doc_of(3)));
    assert_eq!(ys.len(), 3);
    assert!(ys[0] > ys[1], "first line {} must be above second {}", ys[0], ys[1]);
    assert!(ys[1] > ys[2]);
    let page = metrics();
    assert!(ys[0] < page.height - page.margin, "baseline must sit inside the top margin");
    assert!(ys[2] > 0.0, "last baseline must stay on the page");
}
