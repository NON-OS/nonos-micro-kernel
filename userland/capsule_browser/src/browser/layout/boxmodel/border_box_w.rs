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

use super::edges_x::edges_x;

// Border-box width within the available px. box-sizing decides whether the
// declared width is the border box (border-box) or the content box, in which
// case padding and border are added on top (content-box, the default).
pub(super) fn border_box_w(style: &Computed, avail: i32) -> i32 {
    let (el, er) = edges_x(style);
    let edges = if style.border_box { 0 } else { el + er };
    let mut w = match style.width {
        // An auto block fills the container; its border box is the available
        // width regardless of box-sizing.
        Size::Auto => avail,
        // A percentage resolves against the container, then adds edges under
        // content-box; capped to the container so padding cannot overflow it,
        // which is exactly the case border-box was invented to avoid.
        Size::Pct(_) | Size::Calc(_, _) => {
            (style.width.resolve(avail).unwrap_or(avail) + edges).clamp(0, avail)
        }
        // A fixed length keeps its true border-box size even past the
        // container, so an explicitly sized box is not silently shrunk.
        Size::Px(p) => p as i32 + edges,
    };
    // max-width caps, then min-width raises. They clamp the border box.
    if let Some(mx) = style.max_width.resolve(avail) {
        w = w.min(mx + edges);
    }
    if let Some(mn) = style.min_width.resolve(avail) {
        w = w.max(mn + edges);
    }
    w.max(0)
}
