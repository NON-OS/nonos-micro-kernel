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

use alloc::string::String;

use crate::browser::html::parse::read_entity::read_entity;

use super::consume::consume;
use super::flush_text::flush_text;
use super::limits::MAX_NODES;
use super::tree::Dom;

pub fn parse(bytes: &[u8]) -> Dom {
    let mut dom = Dom::new();
    let Ok(text) = core::str::from_utf8(bytes) else {
        return dom;
    };
    let mut cur = 0usize;
    let mut buf = String::new();
    let mut chars = text.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if dom.nodes.len() >= MAX_NODES {
            break;
        }
        match c {
            '<' => {
                flush_text(&mut dom, cur, &mut buf);
                cur = consume(&mut dom, cur, &mut chars);
            }
            '&' => read_entity(&mut chars, &mut buf),
            _ => buf.push(c),
        }
    }
    flush_text(&mut dom, cur, &mut buf);
    dom
}
