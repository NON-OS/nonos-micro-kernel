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

use alloc::vec::Vec;

use crate::doc::block::Block;
use crate::doc::document::Doc;
use crate::doc::kind::BlockKind;
use crate::doc::style::RunStyle;

fn heading_level(line: &str) -> u8 {
    let hashes = line.bytes().take_while(|c| *c == b'#').count();
    if hashes >= 1 && hashes <= 6 && line.as_bytes().get(hashes) == Some(&b' ') {
        hashes as u8
    } else {
        0
    }
}

pub fn doc_from_text(bytes: &[u8]) -> Doc {
    let text = core::str::from_utf8(bytes).unwrap_or("");
    let mut d = Doc::new();
    for line in text.split('\n') {
        let n = heading_level(line);
        if n > 0 {
            let body = &line[(n as usize) + 1..];
            d.blocks.push(Block::plain(BlockKind::Heading(n), body, RunStyle::heading(n)));
        } else {
            d.blocks.push(Block::plain(BlockKind::Paragraph, line, RunStyle::body()));
        }
    }
    if d.blocks.is_empty() {
        d.blocks.push(Block::plain(BlockKind::Paragraph, "", RunStyle::body()));
    }
    d
}

pub fn text_from_doc(doc: &Doc) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for (i, b) in doc.blocks.iter().enumerate() {
        if i > 0 {
            out.push(b'\n');
        }
        if let Some(n) = b.kind.heading_level() {
            for _ in 0..n {
                out.push(b'#');
            }
            out.push(b' ');
        }
        out.extend_from_slice(&b.text);
    }
    out
}
