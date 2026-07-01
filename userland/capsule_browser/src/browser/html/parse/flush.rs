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

pub fn flush(out: &mut Vec<Flow>, buf: &mut String, style: Style, link: &Option<String>) {
    let t = if style.pre { buf.as_str() } else { buf.trim() };
    if !t.is_empty() {
        match link {
            Some(href) => out.push(Flow::Link(t.into(), href.clone())),
            None => out.push(Flow::Text(t.into(), style)),
        }
    }
    buf.clear();
}
