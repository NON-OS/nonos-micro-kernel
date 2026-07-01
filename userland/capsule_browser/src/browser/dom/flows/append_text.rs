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
use crate::browser::html::parse::flush::flush;
use crate::browser::html::parse::push_ws::push_ws;

pub fn append_text(out: &mut Vec<Flow>, buf: &mut String, text: &str, style: Style, link: &Option<String>) {
    for c in text.chars() {
        if style.pre {
            if c == '\n' {
                flush(out, buf, style, link);
                out.push(Flow::Break);
            } else {
                buf.push(c);
            }
        } else if c.is_whitespace() {
            push_ws(buf);
        } else {
            buf.push(c);
        }
    }
}
