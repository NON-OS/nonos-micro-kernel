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

//! The left navigation rail: brand, nav list, account footer. Only "Home" has
//! a screen behind it, so every other row is painted with a sunk label even
//! when it holds the selection.

use nonos_app_skeleton::PaintBuffer;

use crate::editor::widget::{nav_row_h, paint_navlist, NavStyle};

use super::brand::paint_brand;
use super::footer::paint_footer;
use super::metrics::{nav_rect, rail_x, BODY, RAIL_W};
use super::palette::{dim, LABEL, NAV_ACCENT, NAV_RING, RAIL_BG, RAIL_LINE, TITLE};
use super::state::{HomeState, NAV_LABELS};

pub(super) fn paint_rail(fb: &mut PaintBuffer, st: &HomeState) {
    let h = fb.height;
    fb.fill_rect(rail_x(), 0, RAIL_W - 1, h, RAIL_BG);
    fb.fill_rect(rail_x() + RAIL_W - 1, 0, 1, h, RAIL_LINE);
    paint_brand(fb);
    paint_nav(fb, st);
    paint_footer(fb, h);
}

fn paint_nav(fb: &mut PaintBuffer, st: &HomeState) {
    let (x, y, w) = nav_rect();
    let rh = nav_row_h(BODY);
    for (i, label) in NAV_LABELS.iter().enumerate() {
        let live = i == 0;
        let style = NavStyle {
            accent: if live { NAV_ACCENT } else { dim(NAV_ACCENT) },
            ring: if live { NAV_RING } else { dim(NAV_RING) },
            label: if live { LABEL } else { dim(LABEL) },
            label_sel: if live { TITLE } else { dim(TITLE) },
            radius: 9,
            pad_x: 14,
        };
        let sel = if i == st.nav { 0 } else { usize::MAX };
        paint_navlist(fb, (x, y + i as u32 * rh, w), &[*label], sel, BODY, &style);
    }
}
