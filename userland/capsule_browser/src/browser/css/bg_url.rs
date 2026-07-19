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

// The background layer captured from a background or background-image
// declaration: either a url() to fetch, or a gradient function kept verbatim
// for the painter to render. Solid colors and unknown values return None.
pub(super) fn bg_url(name: &str, value: &str) -> Option<String> {
    if name != "background" && name != "background-image" {
        return None;
    }
    if let Some(start) = value.find("url(") {
        let rest = &value[start + 4..];
        let end = rest.find(')')?;
        let inner = rest[..end].trim().trim_matches('"').trim_matches('\'').trim();
        if !inner.is_empty() && !inner.starts_with("data:") {
            return Some(inner.to_string());
        }
    }
    // A gradient function is kept as-is; the painter parses and draws it.
    if let Some(start) = value.find("linear-gradient(").or_else(|| value.find("radial-gradient(")) {
        let rest = &value[start..];
        let open = rest.find('(')?;
        // matching_paren returns rest.len() for unbalanced input, so the naive
        // `open + 1 + end + 1` can exceed rest.len() and panic on the slice
        // (a hostile stylesheet/inline style would abort the capsule). Clamp
        // the end to the string length.
        let end = super::matching_paren::matching_paren(&rest[open + 1..]);
        let stop = (open + 1 + end + 1).min(rest.len());
        return Some(rest[..stop].trim().to_string());
    }
    None
}
