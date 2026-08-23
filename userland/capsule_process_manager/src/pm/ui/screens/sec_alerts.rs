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

use crate::pm::format::u32_decimal;
use crate::pm::state::State;
use crate::pm::theme::{BACKGROUND, CARD_BG, CARD_BORDER, MUTED, TITLE};

use super::super::chrome::Rect;
use super::super::metrics::{
    BODY_PX, CHIP_H, CHIP_PAD_X, CHIP_RADIUS, PANEL_HEAD_H, PANEL_PAD, PANEL_RADIUS,
};
use super::super::text;
use super::{sec_geom, sec_row};

pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let y = r.y + sec_geom::list_y();
    let h = r.h.saturating_sub(sec_geom::list_y());
    fb.fill_round(r.x, y, r.w, h, PANEL_RADIUS, CARD_BG);
    fb.stroke_round(r.x, y, r.w, h, PANEL_RADIUS, 1, CARD_BORDER);
    let top = text::centred_top(y, PANEL_HEAD_H, BODY_PX);
    text::left(fb, r.x + PANEL_PAD, top, b"FINDINGS", TITLE, BODY_PX);
    headline(state, fb, r.x + r.w.saturating_sub(PANEL_PAD), y);
    if state.alerts.is_empty() {
        let clear = b"Nothing holds an authority the monitor did not expect.";
        text::left(fb, r.x + PANEL_PAD, y + PANEL_HEAD_H, clear, MUTED, BODY_PX);
        return;
    }
    for slot in 0..sec_geom::visible(r.h) {
        match state.alerts.get(state.alert_scroll + slot) {
            Some(alert) => sec_row::paint(state, fb, r, alert, slot),
            None => break,
        }
    }
}

// The worst level and how many findings sit at it, as one pill. Level derives Ord
// worst-last, so max() is the headline without a ranking table of its own.
fn headline(state: &State, fb: &mut PaintBuffer, right: u32, y: u32) {
    let level = match state.alerts.iter().map(|a| a.level).max() {
        Some(level) => level,
        None => return,
    };
    let mut buf = [0u8; 12];
    let count = state.alerts.iter().filter(|a| a.level == level).count();
    let n = u32_decimal(count as u32, &mut buf);
    let label = level.label();
    let w = text::width(fb, label, BODY_PX) + text::mono_width(fb, &buf[..n], BODY_PX);
    let pill_w = w + CHIP_PAD_X * 3;
    let x = right.saturating_sub(pill_w);
    let pill_y = y + (PANEL_HEAD_H - CHIP_H) / 2;
    fb.fill_round(x, pill_y, pill_w, CHIP_H, CHIP_RADIUS, sec_row::tint(level));
    let top = text::centred_top(pill_y, CHIP_H, BODY_PX);
    let after = text::left(fb, x + CHIP_PAD_X, top, label, BACKGROUND, BODY_PX).max(0) as u32;
    text::mono(fb, after + CHIP_PAD_X, top, &buf[..n], BACKGROUND, BODY_PX);
}
