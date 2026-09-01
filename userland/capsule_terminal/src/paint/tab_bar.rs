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

use super::tab_chip::draw_chip;
use super::tab_label::{tab_label, LABEL_CAP};
use super::tab_pill::{draw_pill, pill_rect, plus_rect, PILL_W, PLUS_W};
use super::tokens::TOOLBAR_ACTIVE;
use super::toolbar::{draw_toolbar, TOOLBAR_W};
use crate::term::state::State;


/// Width of the accessory left to the tabs and the new-tab chip once the
/// feature buttons have taken their right-aligned share.
pub fn tabs_avail(acc_w: u32) -> u32 {
    acc_w.saturating_sub(TOOLBAR_W)
}

/// Accessory width the bar asks the frame for, before the frame clamps it.
pub fn nominal_w(tabs: usize) -> u32 {
    tabs as u32 * PILL_W + PLUS_W + TOOLBAR_W
}

pub fn draw_tab_bar(tabs: &[State], active: usize, fb: &mut PaintBuffer) {
    let avail = tabs_avail(fb.width);
    let mut buf = [0u8; LABEL_CAP];
    for (i, tab) in tabs.iter().enumerate() {
        let (name_len, n) = tab_label(i, tab, &mut buf);
        draw_pill(fb, pill_rect(i, avail), &buf[..n], name_len, i == active);
    }
    draw_chip(fb, plus_rect(tabs.len(), avail), "+", TOOLBAR_ACTIVE);
    let w = fb.width;
    draw_toolbar(fb, w, None);
}
