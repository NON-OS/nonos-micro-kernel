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

use super::skip_ws;

pub fn read_value(raw: &str, start: usize) -> (Option<String>, usize) {
    let mut i = skip_ws::skip_ws(raw, start);
    if raw.as_bytes().get(i) != Some(&b'=') {
        return (None, i);
    }
    i = skip_ws::skip_ws(raw, i + 1);
    match raw.as_bytes().get(i).copied() {
        Some(b'"') | Some(b'\'') => {
            let quote = raw.as_bytes()[i];
            let from = i + 1;
            let rel = raw[from..].bytes().position(|b| b == quote);
            match rel {
                Some(n) => (Some(raw[from..from + n].to_string()), from + n + 1),
                None => (None, raw.len()),
            }
        }
        Some(_) => {
            let end = match raw[i..].bytes().position(|b| b.is_ascii_whitespace() || b == b'/') {
                Some(n) => i + n,
                None => raw.len(),
            };
            (Some(raw[i..end].to_string()), end)
        }
        None => (None, raw.len()),
    }
}
