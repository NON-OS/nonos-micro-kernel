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

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use super::block::{Block, Style};
use super::heading::{body, heading};

pub fn parse(markdown: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut open: Option<Block> = None;
    let mut depth: u32 = 0;
    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                start(&mut blocks, &mut open, heading(level))
            }
            Event::Start(Tag::Paragraph) => start(&mut blocks, &mut open, body(depth > 0)),
            Event::Start(Tag::Item) => {
                depth += 1;
                start(&mut blocks, &mut open, Style::Bullet);
            }
            Event::Start(Tag::CodeBlock(_)) => start(&mut blocks, &mut open, Style::Code),
            Event::End(TagEnd::Item) => {
                depth = depth.saturating_sub(1);
                flush(&mut blocks, open.take());
            }
            Event::End(TagEnd::Heading(_) | TagEnd::Paragraph | TagEnd::CodeBlock) => {
                flush(&mut blocks, open.take())
            }
            Event::Text(text) => push(&mut open, &text, false),
            Event::Code(text) => push(&mut open, &text, true),
            Event::SoftBreak | Event::HardBreak => push(&mut open, " ", false),
            _ => {}
        }
    }
    flush(&mut blocks, open.take());
    blocks
}

fn start(blocks: &mut Vec<Block>, open: &mut Option<Block>, style: Style) {
    flush(blocks, open.take());
    *open = Some(Block::new(style));
}

fn push(open: &mut Option<Block>, text: &str, mono: bool) {
    if let Some(block) = open.as_mut() {
        block.push(text, mono);
    }
}

fn flush(blocks: &mut Vec<Block>, block: Option<Block>) {
    if let Some(block) = block.filter(|block| !block.is_blank()) {
        blocks.push(block);
    }
}
