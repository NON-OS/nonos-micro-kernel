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

use crate::browser::css::{AutoRepeat, Computed, GridTrack};

// How many times an auto-fill or auto-fit track repeats across `w`. Each
// repetition costs its floor plus a gap, and the last one needs no gap after
// it, so the count is (w + gap) / (floor + gap). None when the container is
// not an auto-repeat grid, or when the floor is a fraction, which gives no
// width to divide by and is not a valid auto-repeat track.
pub(super) fn auto_repeat_n(style: &Computed, w: i32, items: usize) -> Option<usize> {
    let mode = style.grid_auto?;
    let gap = style.gap as i32;
    let floor = match style.grid_auto_min {
        GridTrack::Px(p) => p as i32,
        GridTrack::Pct(p) => w.saturating_mul(p.min(100) as i32) / 100,
        GridTrack::Fr(_) => return None,
    }
    .max(1);
    let cap = style.grid_cols.len() as i32;
    let n = ((w + gap) / (floor + gap)).clamp(1, cap) as usize;
    // auto-fit drops the tracks no item lands in, so the items that do exist
    // share the whole width rather than leaving a gap at the end of the row.
    Some(match mode {
        AutoRepeat::Fit => n.min(items.max(1)),
        AutoRepeat::Fill => n,
    })
}
