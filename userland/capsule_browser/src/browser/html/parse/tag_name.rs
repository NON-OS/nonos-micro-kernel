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
use alloc::string::ToString;

pub fn tag_name(raw: &str) -> String {
    let body = match raw.strip_prefix('/') {
        Some(value) => value,
        None => raw,
    };
    match body.split(|c: char| c.is_whitespace() || c == '/').find(|s| !s.is_empty()) {
        Some(value) => value.to_ascii_lowercase().to_string(),
        None => String::new(),
    }
}
