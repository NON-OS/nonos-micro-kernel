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
                flush(&mut out, &mut buf, style, &link);
                tags += 1;
                consume_tag(text, &mut chars, &mut out, &mut style, &mut link);
            }
            '&' => read_entity(&mut chars, &mut buf),
            c if c.is_whitespace() => push_ws(&mut buf),
            _ => buf.push(c),
        }
    }
    flush(&mut out, &mut buf, style, &link);
    out
}

fn push_ws(buf: &mut String) {
    if !buf.ends_with(' ') {
        buf.push(' ');
    }
}

fn flush(out: &mut Vec<Flow>, buf: &mut String, style: Style, link: &Option<String>) {
    let t = buf.trim();
    if !t.is_empty() {
        match link {
            Some(href) => out.push(Flow::Link(t.into(), href.clone())),
            None => out.push(Flow::Text(t.into(), style)),
        }
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

fn read_to_gt(chars: &mut core::iter::Peekable<core::str::CharIndices>) -> String {
    let mut raw = String::new();
    while let Some(&(_, c)) = chars.peek() {
        chars.next();
        if c == '>' {
            break;
        }
        if raw.len() < 8192 {
            raw.push(c);
        }
    }
    raw
}

fn tag_name(raw: &str) -> String {
    let body = raw.strip_prefix('/').unwrap_or(raw);
    body.split(|c: char| c.is_whitespace() || c == '/')
        .find(|s| !s.is_empty())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn attr(raw: &str, key: &str) -> Option<String> {
    let lower = raw.to_ascii_lowercase();
    let mut from = 0usize;
    while let Some(rel) = lower.get(from..)?.find(key) {
        let at = from + rel;
        from = at + key.len();
        let rest = raw.get(from..)?.trim_start();
        let after = rest.strip_prefix('=')?.trim_start();
        let (q, body) = match after.as_bytes().first() {
            Some(b'"') => ('"', after.get(1..)?),
            Some(b'\'') => ('\'', after.get(1..)?),
            _ => return Some(after.split(|c: char| c.is_whitespace()).next()?.into()),
        };
        return Some(body.split(q).next()?.into());
    }
    None
}

fn skip_until_close(chars: &mut core::iter::Peekable<core::str::CharIndices>, name: &str) {
    let mut scanned = 0u32;
    while let Some((_, c)) = chars.next() {
        scanned = scanned.saturating_add(1);
        if scanned > 4_000_000 {
            break;
        }
        if c == '<' && chars.peek().map(|&(_, n)| n) == Some('/') {
            let raw = read_to_gt(chars);
            if tag_name(&raw) == name {
                break;
            }
        }
    }
}

fn consume_tag(
    text: &str,
    chars: &mut core::iter::Peekable<core::str::CharIndices>,
    out: &mut Vec<Flow>,
    style: &mut Style,
    link: &mut Option<String>,
) {
    let raw = read_to_gt(chars);
    let closing = raw.starts_with('/');
    let name = tag_name(&raw);
    match name.as_str() {
        "br" | "p" | "div" | "li" | "ul" | "tr" | "h1" | "h2" | "h3" | "h4" | "h5"
        | "h6" => out.push(Flow::Break),
        "b" | "strong" => style.bold = !closing,
        "a" => *link = if closing { None } else { attr(&raw, "href") },
        "img" => {
            let src = attr(&raw, "src").unwrap_or_default();
            let alt = attr(&raw, "alt").unwrap_or_default();
            out.push(Flow::Image(src, alt));
        }
        "script" | "style" if !closing => skip_until_close(chars, &name),
        _ => {}
    }
    if name.starts_with('h') && name.len() == 2 && name.as_bytes()[1].is_ascii_digit() {
        style.heading = if closing { 0 } else { name.as_bytes()[1] - b'0' };
    }
    let _ = text;
}
