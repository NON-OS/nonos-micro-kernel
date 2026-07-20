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

//! RFC 3986 section 5.2.4 dot-segment removal for a resolved path. A relative
//! reference merged onto the base can still contain `.` and `..` segments
//! (`/a/b/../d`); without collapsing them the browser fetches the wrong path.
//! `..` pops the previous segment but never climbs above the root, `.` drops,
//! and a trailing slash is preserved.

use alloc::string::String;
use alloc::vec::Vec;

pub fn remove_dot_segments(path: &str) -> String {
    let trailing = path.ends_with('/') && path.len() > 1;
    let mut segs: Vec<&str> = Vec::new();
    for seg in path.split('/').filter(|s| !s.is_empty()) {
        match seg {
            "." => {}
            ".." => {
                segs.pop();
            }
            s => segs.push(s),
        }
    }
    let mut out = String::from("/");
    out.push_str(&segs.join("/"));
    if trailing && !segs.is_empty() {
        out.push('/');
    }
    out
}
