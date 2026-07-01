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

use alloc::string::ToString;
use alloc::vec::Vec;

use crate::browser::css::selector::Simple;

pub fn parse_simple(tok: &str) -> Simple {
    let b = tok.as_bytes();
    let mut i = 0;
    while i < b.len() && b[i] != b'.' && b[i] != b'#' {
        i += 1;
    }
    let tag = match &tok[..i] {
        "" | "*" => None,
        t => Some(t.to_ascii_lowercase()),
    };
    let mut id = None;
    let mut classes: Vec<alloc::string::String> = Vec::new();
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
            id = Some(name.to_string());
        } else if classes.len() < 16 {
            classes.push(name.to_string());
        }
    }
    Simple { tag, id, classes }
}
