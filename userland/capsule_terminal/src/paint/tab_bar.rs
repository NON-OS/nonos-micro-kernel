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
use super::tab_pill::{draw_pill, pill_rect, plus_rect, PILL_H, PILL_W, PLUS_W};
use super::tokens::{TOOLBAR_ACTIVE, TOOLBAR_ICON};
use crate::layout::Rect;
use crate::term::state::State;

pub const FEAT_W: u32 = 34;
pub const FEATURES: [&str; 4] = ["thm", "A-", "A+", "clr"];

/// Width of the accessory left to the tabs and the new-tab chip once the
/// feature buttons have taken their right-aligned share.
pub fn tabs_avail(acc_w: u32) -> u32 {
    acc_w.saturating_sub(FEATURES.len() as u32 * FEAT_W)
}

pub fn feat_rect(i: usize, acc_w: u32) -> Rect {
    Rect { x: tabs_avail(acc_w) + i as u32 * FEAT_W, y: 0, w: FEAT_W - 4, h: PILL_H }
}

/// Which feature button `x` falls on, or None when it lands left of them.
pub fn feat_hit(x: u32, acc_w: u32) -> Option<usize> {
    let base = tabs_avail(acc_w);
    if x < base {
        return None;
    }
    let i = ((x - base) / FEAT_W) as usize;
    (i < FEATURES.len()).then_some(i)
}

/// Accessory width the bar asks the frame for, before the frame clamps it.
pub fn nominal_w(tabs: usize) -> u32 {
    tabs as u32 * PILL_W + PLUS_W + FEATURES.len() as u32 * FEAT_W
}

pub fn draw_tab_bar(tabs: &[State], active: usize, fb: &mut PaintBuffer) {
    let avail = tabs_avail(fb.width);
    let mut buf = [0u8; LABEL_CAP];
    for (i, tab) in tabs.iter().enumerate() {
        let (name_len, n) = tab_label(i, tab, &mut buf);
        draw_pill(fb, pill_rect(i, avail), &buf[..n], name_len, i == active);
    }
    draw_chip(fb, plus_rect(tabs.len(), avail), "+", TOOLBAR_ACTIVE);
    for (i, label) in FEATURES.iter().enumerate() {
        draw_chip(fb, feat_rect(i, fb.width), label, TOOLBAR_ICON);
    }
}
