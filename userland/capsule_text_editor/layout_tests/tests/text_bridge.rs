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

use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::text_bridge::{doc_from_text, text_from_doc};

#[test]
fn each_line_becomes_a_block() {
    let d = doc_from_text(b"one\ntwo\nthree");
    assert_eq!(d.blocks.len(), 3);
    assert_eq!(d.blocks[1].as_str(), "two");
    assert!(d.covered());
}

#[test]
fn round_trip_preserves_the_bytes() {
    let src: &[u8] = b"one\ntwo\n\nfour";
    assert_eq!(text_from_doc(&doc_from_text(src)), src.to_vec());
}

#[test]
fn empty_input_is_one_empty_paragraph() {
    let d = doc_from_text(b"");
    assert_eq!(d.blocks.len(), 1);
    assert_eq!(d.blocks[0].as_str(), "");
    assert!(d.covered());
}

#[test]
fn markdown_hashes_become_headings() {
    let d = doc_from_text(b"# Title\n## Sub\nbody");
    assert_eq!(d.blocks[0].kind, BlockKind::Heading(1));
    assert_eq!(d.blocks[0].as_str(), "Title");
    assert_eq!(d.blocks[1].kind, BlockKind::Heading(2));
    assert_eq!(d.blocks[2].kind, BlockKind::Paragraph);
}

#[test]
fn headings_round_trip_back_to_hashes() {
    let src: &[u8] = b"# Title\nbody";
    assert_eq!(text_from_doc(&doc_from_text(src)), src.to_vec());
}

#[test]
fn invalid_utf8_does_not_panic() {
    let d = doc_from_text(&[0xFF, 0xFE, b'\n', b'a']);
    assert!(d.blocks.len() >= 1);
}
