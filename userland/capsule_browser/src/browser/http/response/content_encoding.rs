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

pub fn content_encoding(value: &str) -> Option<&'static str> {
    let lower = value.trim().to_ascii_lowercase();
    let mut out = "";
    for token in lower.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        match token {
            "identity" => {}
            "gzip" | "x-gzip" => out = "gzip",
            "deflate" if out.is_empty() => out = "deflate",
            _ => return None,
        }
    }
    Some(out)
}
