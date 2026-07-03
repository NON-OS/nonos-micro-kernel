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

use super::default_port::default_port;
use super::types::Scheme;

pub fn host_port(authority: &str, scheme: Scheme) -> Option<(String, u16)> {
    if authority.is_empty()
        || authority.contains('@')
        || authority.bytes().any(|b| b.is_ascii_whitespace())
    {
        return None;
    }
    if authority.starts_with('[') {
        return None;
    }
    if authority.as_bytes().iter().filter(|&&b| b == b':').count() > 1 {
        return None;
    }
    match authority.rsplit_once(':') {
        Some((h, p)) => {
            if h.is_empty() || p.is_empty() {
                return None;
            }
            let port = p.parse().ok()?;
            (port != 0).then(|| (h.to_string(), port))
        }
        _ => Some((authority.to_string(), default_port(scheme))),
    }
}
