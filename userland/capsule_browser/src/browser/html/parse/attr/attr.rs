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

use super::{read_name, read_value, skip_ws};

pub fn attr(raw: &str, key: &str) -> Option<String> {
    let mut i = skip_ws::skip_ws(raw, 0);
    let body = match raw.strip_prefix('/') {
        Some(value) => value,
        None => raw,
    };
    let (_, next) = read_name::read_name(body, i)?;
    i = next;
    while i < raw.len() {
        i = skip_ws::skip_ws(raw, i);
        if i >= raw.len() || raw.as_bytes()[i] == b'/' {
            break;
        }
        let (name, after_name) = match read_name::read_name(raw, i) {
            Some(v) => v,
            None => break,
        };
        let (value, after_value) = read_value::read_value(raw, after_name);
        if name.eq_ignore_ascii_case(key) {
            return value;
        }
        i = after_value.max(after_name + 1);
    }
    None
}
