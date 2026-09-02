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

use crate::term::cwd::{home_var, strip_home};
use crate::term::state::State;

/// Bytes a tab label is built into. The bar owns one buffer for the whole
/// frame, so labelling never allocates.
pub const LABEL_CAP: usize = 96;

const SEP: &str = " \u{2022} ";

/// Session name for tab `i`: `local` for the first, `shell-N` after it. Kept
/// separate so a user-set name can replace it without touching the label.
pub fn session_name(i: usize, out: &mut [u8]) -> usize {
    if i == 0 {
        return copy(out, b"local");
    }
    let n = copy(out, b"shell-");
    if i < 10 && n < out.len() {
        out[n] = b'0' + i as u8;
        return n + 1;
    }
    n
}

/// Writes `<name> • <path>` for tab `i` into `out`, the cwd's `$HOME` prefix
/// collapsed to `~`. Returns the name length and the total, so the pill can
/// drop the path half before it ever cuts the name.
pub fn tab_label(i: usize, tab: &State, out: &mut [u8; LABEL_CAP]) -> (usize, usize) {
    let name_len = session_name(i, &mut out[..]);
    let mut n = name_len + copy(&mut out[name_len..], SEP.as_bytes());
    let cwd = tab.cwd.as_bytes();
    match strip_home(cwd, home_var(tab)) {
        Some(tail) => {
            n += copy(&mut out[n..], b"~");
            n += copy(&mut out[n..], tail);
        }
        None => n += copy(&mut out[n..], cwd),
    }
    (name_len, n)
}

fn copy(out: &mut [u8], src: &[u8]) -> usize {
    let mut n = src.len().min(out.len());
    while n > 0 && n < src.len() && src[n] & 0xC0 == 0x80 {
        n -= 1;
    }
    out[..n].copy_from_slice(&src[..n]);
    n
}
