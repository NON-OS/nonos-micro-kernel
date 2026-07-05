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

// The url() target of a background or background-image declaration, with the
// quotes and whitespace stripped. Gradients and other image functions return
// None, so only a fetchable image is captured.
pub(super) fn bg_url(name: &str, value: &str) -> Option<String> {
    if name != "background" && name != "background-image" {
        return None;
    }
    let start = value.find("url(")? + 4;
    let rest = &value[start..];
    let end = rest.find(')')?;
    let inner = rest[..end].trim().trim_matches('"').trim_matches('\'').trim();
    if inner.is_empty() || inner.starts_with("data:") {
        return None;
    }
    Some(inner.to_string())
}
