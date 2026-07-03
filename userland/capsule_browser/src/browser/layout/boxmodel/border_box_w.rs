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

use crate::browser::css::{Computed, Size};

// Border-box width within the available px. width is treated as the
// border-box size, which keeps real pages inside their containers.
pub(super) fn border_box_w(style: &Computed, avail: i32) -> i32 {
    let mut w = match style.width {
        Size::Auto => avail,
        Size::Px(p) => (p as i32).min(avail),
        s => s.resolve(avail).unwrap_or(avail).clamp(0, avail),
    };
    // max-width caps, then min-width raises; percentages resolve against the
    // available width like the base size does.
    if let Some(mx) = style.max_width.resolve(avail) {
        w = w.min(mx);
    }
    if let Some(mn) = style.min_width.resolve(avail) {
        w = w.max(mn);
    }
    w.max(0)
}
