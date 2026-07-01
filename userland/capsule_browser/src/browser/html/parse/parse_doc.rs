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
use alloc::vec::Vec;

use crate::browser::html::flow::{Flow, Style};
use crate::browser::html::parse::{consume_tag, flush, push_ws, read_entity};

const MAX_TAGS: u32 = 20000;
const MAX_FLOW: usize = 40000;

pub fn parse(bytes: &[u8]) -> Vec<Flow> {
    let Ok(text) = core::str::from_utf8(bytes) else { return Vec::new() };
    let mut out: Vec<Flow> = Vec::new();
    let mut buf = String::new();
    let mut style = Style::default();
    let mut link: Option<String> = None;
    let mut tags = 0u32;
    let mut chars = text.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if out.len() >= MAX_FLOW || tags >= MAX_TAGS {
            break;
        }
        match c {
            '<' => {
                flush::flush(&mut out, &mut buf, style, &link);
                tags += 1;
                consume_tag::consume_tag(&mut chars, &mut out, &mut style, &mut link);
            }
            '&' => read_entity::read_entity(&mut chars, &mut buf),
            c if c.is_whitespace() && style.pre && c == '\n' => {
                flush::flush(&mut out, &mut buf, style, &link);
                out.push(Flow::Break);
            }
            c if c.is_whitespace() && style.pre => buf.push(c),
            c if c.is_whitespace() => push_ws::push_ws(&mut buf),
            _ => buf.push(c),
        }
    }
    flush::flush(&mut out, &mut buf, style, &link);
    out
}
