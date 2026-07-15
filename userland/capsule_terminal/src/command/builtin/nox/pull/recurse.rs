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

use alloc::vec::Vec;

pub struct Entry {
    pub name: Vec<u8>,
    pub is_dir: bool,
}

pub fn parse_autoindex(html: &[u8]) -> Vec<Entry> {
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(rel) = find_from(html, i, b"href=\"") {
        let start = rel + 6;
        let Some(end_rel) = html[start..].iter().position(|&c| c == b'"') else {
            break;
        };
        let href = &html[start..start + end_rel];
        i = start + end_rel + 1;
        if href.is_empty() || href[0] == b'/' || href.starts_with(b"..") || href[0] == b'?' {
            continue;
        }
        out.push(Entry { name: href.to_vec(), is_dir: href.last() == Some(&b'/') });
    }
    out
}

fn find_from(hay: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    if from >= hay.len() {
        return None;
    }
    super::scan::find(&hay[from..], needle).map(|p| p + from)
}
