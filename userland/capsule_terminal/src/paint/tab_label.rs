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

/// Label bytes for tab `i`: its 1-based shortcut digit and the working
/// directory's basename. Length is not capped here; the pill cuts it by
/// measured width.
pub fn tab_label(i: usize, cwd: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(b'1' + i as u8);
    out.push(b':');
    out.push(b' ');
    out.extend_from_slice(basename(cwd));
    out
}

fn basename(path: &[u8]) -> &[u8] {
    match path.iter().rposition(|&b| b == b'/') {
        Some(i) if i + 1 < path.len() => &path[i + 1..],
        _ => path,
    }
}
