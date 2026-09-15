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

extern crate alloc;

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::human_size::human_size;
use super::info_row::kv;

// A failed walk renders a dash on every row rather than dropping the rows, so
// the panel's shape does not change when the server cannot answer.
const UNKNOWN: &str = "--";

/// The occupancy of a directory, summed by the vfs and handed in already walked
/// by `info_cache`. A `truncated` reply means the server hit its walk cap, so
/// the size is shown as a lower bound.
pub fn dir_body(
    fb: &mut PaintBuffer,
    stat: Option<(u32, u32, u64, bool)>,
    x: u32,
    top: u32,
    w: u32,
) -> u32 {
    let mut y = top;
    match stat {
        Some((files, dirs, bytes, truncated)) => {
            y += kv(fb, x, y, w, "Files", &alloc::format!("{files}"));
            y += kv(fb, x, y, w, "Folders", &alloc::format!("{dirs}"));
            y += kv(fb, x, y, w, "Size", &summed(bytes, truncated));
        }
        None => {
            y += kv(fb, x, y, w, "Files", UNKNOWN);
            y += kv(fb, x, y, w, "Folders", UNKNOWN);
            y += kv(fb, x, y, w, "Size", UNKNOWN);
        }
    }
    y
}

// A truncated walk is a lower bound, marked with a leading tilde.
fn summed(bytes: u64, truncated: bool) -> String {
    let size = human_size(bytes);
    if truncated {
        alloc::format!("~{size}")
    } else {
        size
    }
}
