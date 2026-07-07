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

// Pull the target out of an @import prelude, the text sitting between the
// @import token and its semicolon. Handles url("x"), url('x'), url(x) and the
// bare "x" / 'x' string form, ignoring any trailing media query the prelude
// may carry after the location.
pub(super) fn import_url(prelude: &str) -> Option<String> {
    let p = prelude.trim();
    if let Some(rest) = p.strip_prefix("url(") {
        let inner = rest.split(')').next()?.trim();
        let inner = inner
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .or_else(|| inner.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
            .unwrap_or(inner);
        return if inner.is_empty() { None } else { Some(String::from(inner)) };
    }
    let quote = p.as_bytes().first().copied()?;
    if quote == b'"' || quote == b'\'' {
        let end = p[1..].find(quote as char)?;
        let s = &p[1..1 + end];
        return if s.is_empty() { None } else { Some(String::from(s)) };
    }
    None
}
