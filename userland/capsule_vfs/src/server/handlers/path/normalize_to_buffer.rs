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

pub(crate) fn normalize_to_buffer(src: &[u8], out: &mut [u8]) -> usize {
    if out.is_empty() {
        return 0;
    }
    out[0] = b'/';
    let mut len = 1;
    let mut pos = 0;
    while pos <= src.len() {
        let start = pos;
        while pos < src.len() && src[pos] != b'/' {
            pos += 1;
        }
        let part = &src[start..pos];
        if part == b".." {
            while len > 1 {
                len -= 1;
                if out[len] == b'/' {
                    break;
                }
            }
        } else if !part.is_empty() && part != b"." {
            if len > 1 && len < out.len() {
                out[len] = b'/';
                len += 1;
            }
            let mut i = 0;
            while i < part.len() && len < out.len() {
                out[len] = part[i];
                len += 1;
                i += 1;
            }
        }
        if pos == src.len() {
            break;
        }
        pos += 1;
    }
    len
}
