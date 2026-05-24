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

use crate::window::{Visibility, Window, WindowTable};

pub struct HitTarget {
    pub owner_pid: u32,
    pub window_id: u32,
    pub local_x: u32,
    pub local_y: u32,
}

pub fn topmost_hit_at(table: &WindowTable, px: u32, py: u32) -> Option<HitTarget> {
    let mut best: Option<&Window> = None;
    for w in table.windows() {
        if w.visibility != Visibility::Visible {
            continue;
        }
        if !w.rect.contains(px, py) {
            continue;
        }
        if !w.kind.focusable() {
            continue;
        }
        best = Some(match best {
            None => w,
            Some(cur) if w.z > cur.z => w,
            Some(cur) => cur,
        });
    }
    best.map(|w| HitTarget {
        owner_pid: w.owner_pid,
        window_id: w.window_id,
        local_x: px.saturating_sub(w.rect.x),
        local_y: py.saturating_sub(w.rect.y),
    })
}
