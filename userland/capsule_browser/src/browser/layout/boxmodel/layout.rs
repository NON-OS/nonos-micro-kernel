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

use super::containing::Containing;
use super::ctx::Ctx;
use super::display_list::{BoxDocument, DisplayList};
use super::layout_box::layout_box;
use super::tree::BoxNode;

// Lay the whole page: the root (body) box against the viewport width. The
// viewport is the initial containing block; fragments sort by stacking z so
// the painter and hit-testing agree on order.
pub fn layout(root: &BoxNode, viewport_w: u32) -> BoxDocument {
    let mut frags: DisplayList = Vec::new();
    let s = &root.style;
    let (ml, mr) = (s.margin_left as i32, s.margin_right as i32);
    let (mt, mb) = (s.margin_top as i32, s.margin_bottom as i32);
    let avail = (viewport_w as i32 - ml - mr).max(0);
    let ctx = Ctx {
        cb: Containing {
            x: 0,
            y: 0,
            w: viewport_w as i32,
            h: Some(crate::browser::manifest::HEIGHT as i32),
        },
        clip: None,
        z: 0,
        fixed: false,
    };
    let h = layout_box(root, ml, mt, avail, &mut frags, 0, ctx);
    frags.sort_by_key(|f| f.z);
    let mut bottom = mt + h + mb;
    for f in &frags {
        bottom = bottom.max(f.y + f.h);
    }
    BoxDocument { frags, content_h: bottom.max(0) as u32 }
}
