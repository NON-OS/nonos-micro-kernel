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
use alloc::vec::Vec;

use crate::browser::html::parse::attr::read_name::read_name;
use crate::browser::html::parse::attr::read_value::read_value;
use crate::browser::html::parse::attr::skip_ws::skip_ws;

pub fn parse_attrs(raw: &str) -> Vec<(String, String)> {
    let body = raw.strip_prefix('/').unwrap_or(raw);
    let mut out: Vec<(String, String)> = Vec::new();
    let mut i = skip_ws(body, 0);
    let Some((_, next)) = read_name(body, i) else {
        return out;
    };
    i = next;
    while i < body.len() && out.len() < 64 {
        i = skip_ws(body, i);
        if i >= body.len() || body.as_bytes()[i] == b'/' {
            break;
        }
        let Some((name, after_name)) = read_name(body, i) else {
            break;
        };
        let (value, after_value) = read_value(body, after_name);
        out.push((name.to_ascii_lowercase(), value.unwrap_or_default().to_string()));
        i = after_value.max(after_name + 1);
    }
    out
}
