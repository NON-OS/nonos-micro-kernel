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
use nonos_toolkit::icons::draw;

use crate::about::data::build::VERSION;
use crate::about::data::product::NAME;
use crate::about::format::trimmed;
use crate::about::section::{Section, SECTIONS};
use crate::about::theme::{
    ACCENT, MUTED, NAV_BG_ACTIVE, NAV_BORDER_ACTIVE, NAV_FG, NAV_FG_ACTIVE, SIDEBAR_BG,
    SIDEBAR_LINE,
};

use super::metrics::{
    BODY_PX, NAV_H, NAV_ICON, NAV_LABEL_GAP, NAV_PAD_X, NAV_RADIUS, SIDEBAR_W, SIDE_FOOT_H,
    SIDE_FOOT_LINE, SIDE_FOOT_TOP,
};
use super::nav_geom::{row_w, row_x, row_y};
use super::text;

pub fn paint(fb: &mut PaintBuffer, active: Section, h: u32) {
    fb.fill_rect(0, 0, SIDEBAR_W, h, SIDEBAR_BG);
    fb.fill_rect(SIDEBAR_W - 1, 0, 1, h, SIDEBAR_LINE);
    for (i, section) in SECTIONS.iter().enumerate() {
        paint_row(fb, *section, row_y(i), *section == active);
    }
    paint_foot(fb, h);
}

fn paint_row(fb: &mut PaintBuffer, section: Section, y: u32, active: bool) {
    let x = row_x();
    if active {
        fb.fill_round(x, y, row_w(), NAV_H, NAV_RADIUS, NAV_BG_ACTIVE);
        fb.stroke_round(x, y, row_w(), NAV_H, NAV_RADIUS, 1, NAV_BORDER_ACTIVE);
    }
    let fg = if active { NAV_FG_ACTIVE } else { NAV_FG };
    let icon_x = x + NAV_PAD_X;
    let icon_y = y + (NAV_H - NAV_ICON) / 2;
    draw(fb, section.icon(), icon_x, icon_y, NAV_ICON, fg);
    let top = text::top_of(y as i32, NAV_H, BODY_PX);
    text::line(fb, icon_x + NAV_ICON + NAV_LABEL_GAP, top, section.nav_label(), fg, BODY_PX);
}

// The footer mark is the window's provenance in two lines: which system this is
// and which image of it, taken from the same VERSION file the build embeds.
fn paint_foot(fb: &mut PaintBuffer, h: u32) {
    let y = h.saturating_sub(SIDE_FOOT_H);
    fb.fill_rect(row_x(), y, row_w(), 1, SIDEBAR_LINE);
    let x = row_x() + NAV_PAD_X;
    text::line(fb, x, (y + SIDE_FOOT_TOP) as i32, NAME, ACCENT, BODY_PX);
    let line = (y + SIDE_FOOT_TOP + SIDE_FOOT_LINE) as i32;
    let advance = text::line(fb, x, line, b"v", MUTED, BODY_PX);
    text::mono(fb, advance.max(0) as u32, line, trimmed(VERSION), MUTED, BODY_PX);
}
