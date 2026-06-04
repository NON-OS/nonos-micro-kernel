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

pub fn resolve(cwd: &[u8], arg: &[u8]) -> Vec<u8> {
    let mut segs: Vec<&[u8]> = Vec::new();
    let base: &[u8] = if arg.first() == Some(&b'/') { b"" } else { cwd };
    for seg in base.split(|&c| c == b'/') {
        if !seg.is_empty() {
            segs.push(seg);
        }
    }
    for seg in arg.split(|&c| c == b'/') {
        match seg {
            b"" | b"." => {}
            b".." => {
                segs.pop();
            }
            other => segs.push(other),
        }
    }
    let mut out = Vec::new();
    for s in &segs {
        out.push(b'/');
        out.extend_from_slice(s);
    }
    if out.is_empty() {
        out.push(b'/');
    }
    out
}
