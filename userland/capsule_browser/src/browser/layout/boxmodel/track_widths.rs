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

use crate::browser::css::{Computed, GridTrack};

use super::auto_repeat_n::auto_repeat_n;

// Resolve the column tracks into (x offset, width) pairs within `w`. Fixed
// tracks take their size, fractions split what remains after the gaps. A
// grid with no template gets one full-width column. `items` is the number of
// in-flow grid items, which only an auto-fit template needs.
pub(super) fn track_widths(style: &Computed, w: i32, items: usize) -> Vec<(i32, i32)> {
    let n = auto_repeat_n(style, w, items)
        .unwrap_or_else(|| (style.grid_col_n as usize).clamp(1, style.grid_cols.len()));
    let gap = style.gap as i32;
    let mut widths: Vec<i32> = Vec::with_capacity(n);
    let mut fr_total = 0i64;
    let mut fixed = gap * (n as i32 - 1);
    for i in 0..n {
        let track = if style.grid_col_n == 0 { GridTrack::Fr(1) } else { style.grid_cols[i] };
        match track {
            GridTrack::Px(p) => {
                let px = (p as i32).min(w);
                widths.push(px);
                fixed += px;
            }
            GridTrack::Pct(p) => {
                let px = w.saturating_mul(p.min(100) as i32) / 100;
                widths.push(px);
                fixed += px;
            }
            GridTrack::Fr(f) => {
                widths.push(-(f.max(1) as i32));
                fr_total += f.max(1) as i64;
            }
        }
    }
    let free = (w - fixed).max(0) as i64;
    for wd in widths.iter_mut() {
        if *wd < 0 {
            let f = (-*wd) as i64;
            *wd = ((free * f) / fr_total.max(1)) as i32;
        }
    }
    let mut out: Vec<(i32, i32)> = Vec::with_capacity(n);
    let mut x = 0i32;
    for wd in widths {
        let wd = wd.max(1);
        out.push((x, wd));
        x += wd + gap;
    }
    out
}
