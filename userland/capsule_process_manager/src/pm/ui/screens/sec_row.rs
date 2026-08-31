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
use crate::pm::security::{Alert, Level};
use crate::pm::state::State;
use crate::pm::theme::{ACCENT, AMBER, BAND, DANGER, FOREGROUND, MUTED, SELECT_BG};

use super::super::chrome::Rect;
use super::super::metrics::{BODY_PX, CHIP_RADIUS, NUM_PX, PANEL_PAD};
use super::super::text;
use super::sec_geom::{self, ALERT_H, LEVEL_BAR_W};

const LINE_H: u32 = 21;
const TOP_PAD: u32 = 3;
const GAP: u32 = 9;
const PID_W: u32 = 76;

pub fn tint(level: Level) -> u32 {
    match level {
        Level::Info => ACCENT,
        Level::Warn => AMBER,
        Level::Critical => DANGER,
    }
}

// BAND is opaque and may be written; SELECT_BG carries alpha and must blend, or
// the card would punch through the panel it sits on. Selection reads the same
// here as in the table: a wash plus an accent bar down the left edge.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect, alert: &Alert, slot: usize) {
    let y = r.y + sec_geom::row_y(slot);
    let x = r.x + PANEL_PAD;
    let w = r.w.saturating_sub(PANEL_PAD * 2);
    let selected = state.alert_scroll + slot == state.alert_sel;
    fb.fill_round(x, y, w, ALERT_H, CHIP_RADIUS, BAND);
    if selected {
        fb.blend_rect(x, y, w, ALERT_H, SELECT_BG);
    }
    let bar = if selected { ACCENT } else { tint(alert.level) };
    fb.fill_rect(x, y, LEVEL_BAR_W, ALERT_H, bar);
    head(fb, alert, x + LEVEL_BAR_W + GAP, y, x + w.saturating_sub(GAP));
    let msg = text::fit(fb, alert.msg, BODY_PX, w.saturating_sub(LEVEL_BAR_W + GAP * 2));
    text::left(fb, x + LEVEL_BAR_W + GAP, y + TOP_PAD + LINE_H, msg, MUTED, BODY_PX);
}

// Level tag, the process the finding is about, and its pid right-aligned. A
// system-wide finding carries pid 0, which prints as a dash rather than a zero
// nobody can look up.
fn head(fb: &mut PaintBuffer, alert: &Alert, x: u32, y: u32, right: u32) {
    let top = y + TOP_PAD;
    let after = text::left(fb, x, top, alert.level.label(), tint(alert.level), BODY_PX);
    let name_x = after.max(0) as u32 + GAP;
    let name = text::fit(fb, alert.name(), BODY_PX, right.saturating_sub(name_x + PID_W));
    text::left(fb, name_x, top, name, FOREGROUND, BODY_PX);
    let mut buf = [0u8; 12];
    let n = u32_decimal(alert.pid, &mut buf);
    let pid: &[u8] = if alert.pid == 0 { b"-" } else { &buf[..n] };
    text::mono_right(fb, right, top, pid, MUTED, NUM_PX);
}
