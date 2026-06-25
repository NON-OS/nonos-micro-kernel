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

use super::entity::push_decoded;
use super::flow::{Flow, Style};

const MAX_TAGS: u32 = 20000;
const MAX_FLOW: usize = 40000;

pub fn parse(bytes: &[u8]) -> Vec<Flow> {
    let text = core::str::from_utf8(bytes).unwrap_or("");
    let mut out: Vec<Flow> = Vec::new();
    let mut buf = String::new();
    let style = Style::default();
    let mut tags = 0u32;
    let mut chars = text.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if out.len() >= MAX_FLOW || tags >= MAX_TAGS {
            break;
        }
        match c {
            '<' => {
                flush(&mut out, &mut buf, style);
                tags += 1;
                consume_tag(text, &mut chars);
            }
            '&' => read_entity(&mut chars, &mut buf),
            c if c.is_whitespace() => push_ws(&mut buf),
            _ => buf.push(c),
        }
    }
    flush(&mut out, &mut buf, style);
    out
}

fn push_ws(buf: &mut String) {
    if !buf.ends_with(' ') {
        buf.push(' ');
    }
}

fn flush(out: &mut Vec<Flow>, buf: &mut String, style: Style) {
    let t = buf.trim();
    if !t.is_empty() {
        out.push(Flow::Text(t.into(), style));
    }
    buf.clear();
}

fn read_entity(chars: &mut core::iter::Peekable<core::str::CharIndices>, buf: &mut String) {
    let mut name = String::new();
    while let Some(&(_, c)) = chars.peek() {
        if c == ';' || name.len() >= 12 {
            chars.next();
            break;
        }
        if !c.is_ascii_alphanumeric() && c != '#' {
            break;
        }
        name.push(c);
        chars.next();
    }
    if name.is_empty() {
        buf.push('&');
    } else {
        push_decoded(buf, &name);
    }
}

fn consume_tag(text: &str, chars: &mut core::iter::Peekable<core::str::CharIndices>) {
    while let Some((_, c)) = chars.next() {
        if c == '>' {
            break;
        }
    }
    let _ = text;
}
