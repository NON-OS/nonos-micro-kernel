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

//! Paint an open panel over the shell: a rounded card, its title, then the
//! rows through the shared nav list so a picker row looks like every other
//! row in the capsule.

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::ttf::line_height;

use super::app::Editor;
use super::layout::CHROME_PX;
use super::panel::{panel_title, Panel, PANEL_PAD, PANEL_R};
use super::panel_geom::{panel_list, panel_rect};
use super::theme;
use super::widget::{nav_row_h, paint_navlist, NavStyle};

const PANEL_SHADOW: u32 = 0x5000_0000;

impl Editor {
    pub(super) fn paint_panel(&self, fb: &mut PaintBuffer, panel: Panel) {
        let owned = self.panel_rows(panel);
        let labels: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
        let title = panel_title(panel);
        let rect = panel_rect(self.last_w, self.last_h, title, &labels);
        let (x, y, w, h) = rect;
        let th = theme::active();
        fb.shadow_round(x, y, w, h, PANEL_R, 12, PANEL_SHADOW);
        fb.fill_round(x, y, w, h, PANEL_R, th.tab_inactive_bg);
        fb.stroke_round(x, y, w, h, PANEL_R, 1, th.line);
        let lh = line_height(CHROME_PX).max(1) as u32;
        let ty = y + PANEL_PAD / 2 + nav_row_h(CHROME_PX).saturating_sub(lh) / 2;
        let _ = fb.text_ttf((x + PANEL_PAD) as i32, ty as i32, title, th.accent, CHROME_PX);
        let st = NavStyle {
            accent: th.row_select,
            ring: th.line,
            label: th.foreground,
            label_sel: th.foreground,
            radius: 6,
            pad_x: PANEL_PAD,
        };
        paint_navlist(fb, panel_list(rect), &labels, usize::MAX, CHROME_PX, &st);
    }
}
