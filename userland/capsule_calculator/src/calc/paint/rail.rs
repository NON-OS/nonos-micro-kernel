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
use nonos_toolkit::font::ttf::line_height;

use crate::calc::mode::MODES;
use crate::calc::state::State;
use crate::calc::theme;
use crate::calc::ui::metrics::{
    BRAND_H, BRAND_TOP, NAV_H, NAV_ICON, NAV_LABEL_GAP, NAV_RADIUS, PX_TITLE, RAIL_PAD_X, RAIL_W,
};
use crate::calc::ui::nav_geom::{row_w, row_x, row_y};

const BRAND: &str = "Calculator";

fn centered_top(top: i32, box_h: i32, px: f32) -> i32 {
    top + (box_h - line_height(px).max(1)) / 2
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let h = fb.height;
    fb.fill_rect(0, 0, RAIL_W as u32, h, theme::VOID);
    fb.blend_rect((RAIL_W - 1) as u32, 0, 1, h, theme::LINE_2);
    fb.text_ttf(
        RAIL_PAD_X,
        centered_top(BRAND_TOP, BRAND_H, PX_TITLE),
        BRAND,
        theme::CYAN,
        PX_TITLE,
    );
    for (i, mode) in MODES.iter().enumerate() {
        let (x, y, w) = (row_x() as u32, row_y(i) as u32, row_w() as u32);
        let active = state.mode == *mode;
        if active {
            fb.fill_round(x, y, w, NAV_H as u32, NAV_RADIUS as u32, theme::GLOW);
            fb.stroke_round(x, y, w, NAV_H as u32, NAV_RADIUS as u32, 1, theme::LINE_3);
        } else if state.hover == Some(*mode) {
            fb.blend_rect(x, y, w, NAV_H as u32, theme::LINE);
        }
        let ink = if active { theme::CYAN } else { theme::DIM };
        let label_x = row_x() + RAIL_PAD_X + NAV_ICON + NAV_LABEL_GAP;
        fb.text_ttf(
            label_x,
            centered_top(row_y(i), NAV_H, PX_TITLE),
            mode.label(),
            ink,
            PX_TITLE,
        );
    }
}
