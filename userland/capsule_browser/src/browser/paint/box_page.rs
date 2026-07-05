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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::layout::boxmodel::BoxDocument;
use crate::browser::state::State;

use super::box_fragment::box_fragment;

pub(super) const TOP: i32 = 80;
const PAGE_BG: u32 = 0xFF18_1B20;

pub fn paint(state: &State, doc: &BoxDocument, fb: &mut PaintBuffer) {
    fb.fill_rect(0, TOP as u32, fb.width, fb.height.saturating_sub(TOP as u32), PAGE_BG);
    let bottom = fb.height as i32;
    for f in &doc.frags {
        // A fixed fragment ignores the scroll offset so it pins to the
        // viewport; everything else scrolls with the page.
        let sy = if f.fixed { f.y + TOP } else { f.y + TOP - state.scroll as i32 };
        if sy + f.h < TOP || sy > bottom {
            continue;
        }
        box_fragment(state, fb, f, sy, bottom);
    }
}
