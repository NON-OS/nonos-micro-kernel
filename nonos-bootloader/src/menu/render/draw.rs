// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::font::draw_string;
use crate::display::gop::get_dimensions;
use crate::menu::brand;
use crate::menu::types::MenuState;
use super::entries::draw_entries;
use super::footer::{draw_bottom_bar, draw_panel_footer};
use super::layout::{get_panel_bounds, ENTRY_H, PAD, PANEL_WIDTH, MARGIN};
use super::panel::{draw_panel_background, draw_panel_header};
pub use super::panel::{clear_menu_area, clear_screen};

pub fn draw_logo() {
    let (sw, _) = get_dimensions();
    let cx = (sw - PANEL_WIDTH - MARGIN * 2) / 2;
    crate::display::draw_wordmark(cx.saturating_sub(40), 120);
    let tw = brand::TAGLINE.len() as u32 * 8;
    draw_string(cx.saturating_sub(tw / 2), 172, brand::TAGLINE, brand::TEXT_SECONDARY);
}

pub fn render_menu(state: &MenuState) {
    if !state.visible { return; }
    let (px, py, pw, _) = get_panel_bounds();
    draw_panel_background(px, py, pw);
    draw_panel_header(px, py, pw);
    let ey = py + 70;
    draw_entries(state, px + PAD, ey, pw - PAD * 2);
    draw_panel_footer(state, px + PAD, ey + (state.entries.len() as u32 * ENTRY_H) + 30, pw - PAD * 2);
    draw_bottom_bar(state);
}
