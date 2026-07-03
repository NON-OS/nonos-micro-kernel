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

use alloc::string::{String, ToString};

use crate::browser::css::selector::Simple;

// Scan "tag.class#id" into the simple's tag, id and class list. :root keys
// on the html element regardless of a written tag.
pub(super) fn parse_compound(tok: &str, pseudo_root: bool, s: &mut Simple) {
    let b = tok.as_bytes();
    let mut i = 0;
    while i < b.len() && b[i] != b'.' && b[i] != b'#' {
        i += 1;
    }
    s.tag = if pseudo_root {
        Some(String::from("html"))
    } else {
        match &tok[..i] {
            "" | "*" => None,
            t => Some(t.to_ascii_lowercase()),
        }
    };
    while i < b.len() {
        let marker = b[i];
        i += 1;
        let start = i;
        while i < b.len() && b[i] != b'.' && b[i] != b'#' {
            i += 1;
        }
        let name = &tok[start..i];
        if name.is_empty() {
            continue;
        }
        if marker == b'#' {
            s.id = Some(name.to_string());
        } else if s.classes.len() < 16 {
            s.classes.push(name.to_string());
        }
    }
}
