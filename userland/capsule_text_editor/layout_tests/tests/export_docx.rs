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
use capsule_text_editor_layout_tests::doc::export::docx::to_docx;
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

const PARTS: [&str; 4] = [
    "[Content_Types].xml",
    "_rels/.rels",
    "word/document.xml",
    "word/_rels/document.xml.rels",
];

fn le16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn central_names(out: &[u8]) -> Vec<String> {
    let e = out.len() - 22;
    let mut at = u32::from_le_bytes(out[e + 16..e + 20].try_into().unwrap()) as usize;
    let mut names = Vec::new();
    for _ in 0..le16(out, e + 10) {
        assert_eq!(&out[at..at + 4], b"PK\x01\x02");
        let n = le16(out, at + 28) as usize;
        names.push(String::from_utf8(out[at + 46..at + 46 + n].to_vec()).unwrap());
        at += 46 + n + le16(out, at + 30) as usize + le16(out, at + 32) as usize;
    }
    assert_eq!(at, e);
    names
}

#[test]
fn container_declares_every_part_once() {
    let out = to_docx(&Doc::new());
    assert_eq!(&out[0..4], b"PK\x03\x04");
    assert_eq!(le16(&out, out.len() - 22 + 10), 4);
    assert_eq!(central_names(&out), PARTS.to_vec());
}

#[test]
fn part_bodies_carry_their_root_elements() {
    let mut doc = Doc::new();
    doc.blocks.push(Block::plain(BlockKind::Paragraph, "hi", RunStyle::body()));
    let out = to_docx(&doc);
    for needle in [
        b"<Types xmlns=".as_slice(),
        b"officeDocument/2006/relationships/officeDocument".as_slice(),
        b"<w:document xmlns:w=".as_slice(),
        b"<w:t xml:space=\"preserve\">hi</w:t>".as_slice(),
    ] {
        assert!(out.windows(needle.len()).any(|w| w == needle));
    }
}
