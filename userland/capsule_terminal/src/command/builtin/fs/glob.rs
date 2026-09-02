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

//! Shell-style name matching for find -name: `*` and `?` wildcards.

pub(super) fn matches(pat: &[u8], name: &[u8]) -> bool {
    match pat.first() {
        None => name.is_empty(),
        Some(b'*') => {
            matches(&pat[1..], name) || (!name.is_empty() && matches(pat, &name[1..]))
        }
        Some(b'?') => !name.is_empty() && matches(&pat[1..], &name[1..]),
        Some(&c) => !name.is_empty() && name[0] == c && matches(&pat[1..], &name[1..]),
    }
}

pub(super) fn basename(path: &[u8]) -> &[u8] {
    let trimmed = if path.last() == Some(&b'/') { &path[..path.len() - 1] } else { path };
    match trimmed.iter().rposition(|&b| b == b'/') {
        Some(i) => &trimmed[i + 1..],
        None => trimmed,
    }
}
