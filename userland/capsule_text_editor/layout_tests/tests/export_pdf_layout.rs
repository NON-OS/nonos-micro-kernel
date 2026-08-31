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
fn the_first_baseline_is_the_ascent_below_the_top_margin() {
    let ys = tm_ys(&pdf_of(&doc_of(1)));
    let ascent = RunStyle::body().size_px * 1.1;
    let expect = metrics().height - metrics().margin - ascent;
    assert!((ys[0] - expect).abs() < 0.01, "got {} want {}", ys[0], expect);
}

#[test]
fn literal_strings_escape_parens_and_backslashes() {
    let mut d = Doc::new();
    d.blocks.push(Block::plain(BlockKind::Paragraph, "a(b)c\\d", RunStyle::body()));
    let src = pdf_of(&d);
    assert!(src.contains("(a\\(b\\)c\\\\d) Tj"), "{}", src);
}

#[test]
fn characters_outside_winansi_become_question_marks() {
    let mut d = Doc::new();
    d.blocks.push(Block::plain(BlockKind::Paragraph, "hi \u{4e2d} \u{2014}", RunStyle::body()));
    let out = to_pdf(&d, &metrics(), &FixedMeasurer);
    let at = out.windows(3).position(|w| w == b"(hi").unwrap();
    assert_eq!(&out[at..at + 8], b"(hi ? \x97)");
}

#[test]
fn the_font_resource_follows_family_and_weight() {
    let mut d = Doc::new();
    let mut mono = RunStyle::body();
    mono.family = Family::Mono;
    mono.bold = true;
    d.blocks.push(Block::plain(BlockKind::Paragraph, "x", mono));
    let src = pdf_of(&d);
    assert!(src.contains("/F4 "), "{}", src);
    assert!(src.contains("/BaseFont /Courier-Bold"));
    assert!(src.contains("/BaseFont /Helvetica /Encoding /WinAnsiEncoding"));
}

#[test]
fn every_xref_offset_lands_on_its_object_header() {
    let out = to_pdf(&doc_of(40), &metrics(), &FixedMeasurer);
    let table = find(&out, b"xref\n0 ");
    let rows = out[table..].split(|b| *b == b'\n').skip(3);
    let mut seen = 0usize;
    for (i, row) in rows.take_while(|r| r.ends_with(b" n ")).enumerate() {
        let off: usize = core::str::from_utf8(&row[..10]).unwrap().parse().unwrap();
        let head = format!("{} 0 obj\n", i + 1);
        assert_eq!(&out[off..off + head.len()], head.as_bytes(), "object {}", i + 1);
        seen += 1;
    }
    assert_eq!(seen, 26);
}
