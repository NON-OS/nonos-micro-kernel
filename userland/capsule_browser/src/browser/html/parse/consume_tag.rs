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
use crate::browser::html::parse::{attr, read_to_gt, skip_until_close, tag_name};

pub fn consume_tag(
    chars: &mut core::iter::Peekable<core::str::CharIndices>,
    out: &mut Vec<Flow>,
    style: &mut Style,
    link: &mut Option<String>,
) {
    let raw = read_to_gt::read_to_gt(chars);
    let closing = raw.starts_with('/');
    let name = tag_name::tag_name(&raw);
    match name.as_str() {
        "li" if !closing => {
            out.push(Flow::Break);
            out.push(Flow::Text(String::from("*"), Style::default()));
        }
        "td" | "th" if !closing => out.push(Flow::Text(String::from("|"), Style::default())),
        "br" | "hr" | "p" | "div" | "li" | "ul" | "tr" | "h1" | "h2" | "h3" | "h4" | "h5"
        | "h6" => out.push(Flow::Break),
        "b" | "strong" => style.bold = !closing,
        "pre" | "code" => style.pre = !closing,
        "a" => *link = if closing { None } else { attr::attr(&raw, "href") },
        "img" => {
            let src = match attr::attr(&raw, "src") {
                Some(value) => value,
                None => String::new(),
            };
            let alt = match attr::attr(&raw, "alt") {
                Some(value) => value,
                None => String::new(),
            };
            out.push(Flow::Image(src, alt));
        }
        "head" | "script" | "style" if !closing => skip_until_close::skip_until_close(chars, &name),
        _ => {}
    }
    if name.starts_with('h') && name.len() == 2 && name.as_bytes()[1].is_ascii_digit() {
        style.heading = if closing { 0 } else { name.as_bytes()[1] - b'0' };
    }
}
