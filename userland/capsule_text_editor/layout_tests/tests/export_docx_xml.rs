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
use capsule_text_editor_layout_tests::doc::export::docx::document_xml;
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

fn styled(px: f32) -> RunStyle {
    RunStyle {
        bold: true,
        italic: true,
        underline: true,
        strike: true,
        size_px: px,
        ..RunStyle::body()
    }
}

#[test]
fn headings_and_lists_get_paragraph_styles() {
    let mut doc = Doc::new();
    doc.blocks.push(Block::plain(BlockKind::Heading(2), "T", RunStyle::heading(2)));
    doc.blocks.push(Block::plain(BlockKind::Bullet, "item", RunStyle::body()));
    doc.blocks.push(Block::plain(BlockKind::Numbered, "step", RunStyle::body()));
    doc.blocks.push(Block::plain(BlockKind::Paragraph, "flat", RunStyle::body()));
    let xml = document_xml(&doc);
    assert!(xml.contains("<w:p><w:pPr><w:pStyle w:val=\"Heading2\"/></w:pPr>"));
    assert!(xml.contains("<w:pStyle w:val=\"ListBullet\"/>"));
    assert!(xml.contains("<w:pStyle w:val=\"ListNumber\"/>"));
    assert_eq!(xml.matches("<w:pStyle").count(), 3);
    assert_eq!(xml.matches("<w:p>").count(), 4);
    assert!(xml.ends_with("</w:body></w:document>"));
}

#[test]
fn run_properties_follow_the_style_model() {
    let mut b = Block::plain(BlockKind::Paragraph, "boldplain", RunStyle::body());
    b.runs = vec![
        Run { len: 4, style: styled(12.0) },
        Run { len: 5, style: RunStyle::body() },
    ];
    assert!(b.covered());
    let mut doc = Doc::new();
    doc.blocks.push(b);
    let xml = document_xml(&doc);
    assert!(xml.contains("<w:b/><w:i/><w:strike/><w:sz w:val=\"24\"/><w:u w:val=\"single\"/>"));
    assert!(xml.contains("<w:t xml:space=\"preserve\">bold</w:t>"));
    assert!(xml.contains("<w:rPr><w:sz w:val=\"32\"/></w:rPr>"));
    assert!(xml.contains("<w:t xml:space=\"preserve\">plain</w:t>"));
}

#[test]
fn text_is_xml_escaped() {
    let mut doc = Doc::new();
    let text = "a & b < c > d \" e";
    doc.blocks.push(Block::plain(BlockKind::Paragraph, text, RunStyle::body()));
    let xml = document_xml(&doc);
    assert!(xml.contains("a &amp; b &lt; c &gt; d &quot; e"));
    assert!(!xml.contains("a & b"));
}
