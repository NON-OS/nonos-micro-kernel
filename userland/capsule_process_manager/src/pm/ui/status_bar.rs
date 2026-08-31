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

use crate::pm::format::{mem_human, u32_decimal};
use crate::pm::security::Level;
use crate::pm::state::State;
use crate::pm::theme::{ACCENT, AMBER, DANGER, HEADER_BG, MUTED, OK, RULE, TITLE};

use super::metrics::{BODY_PX, PANE_PAD_X, STATUS_GAP, STATUS_GROUP_GAP, STATUS_H};
use super::text;

// Every number the old footer totals line carried, on one strip: how many
// processes the snapshot holds, what they cost, and the worst thing the monitor
// found in them. The sort column sits on the right because it describes the
// table rather than the system.
pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let y = fb.height.saturating_sub(STATUS_H);
    fb.fill_rect(0, y, fb.width, STATUS_H, HEADER_BG);
    fb.fill_rect(0, y, fb.width, 1, RULE);
    let top = text::centred_top(y, STATUS_H, BODY_PX);
    let mut buf = [0u8; 24];
    let total = state.rows.len() as u32;
    let visible = state.filtered().len() as u32;
    let mut n = u32_decimal(total, &mut buf);
    if visible != total {
        n = u32_decimal(visible, &mut buf);
        buf[n] = b'/';
        n += 1;
        n += u32_decimal(total, &mut buf[n..]);
    }
    let mut x = pair(fb, PANE_PAD_X, top, b"PROCS", &buf[..n]);
    let n = u32_decimal(state.total_cpu, &mut buf);
    x = pair(fb, x, top, b"CPU", &buf[..n]);
    let n = mem_human(state.total_mem_kb, &mut buf);
    x = pair(fb, x, top, b"MEM", &buf[..n]);
    let (tone, label) = posture(state);
    text::left(fb, x, top, label, tone, BODY_PX);
    let right_x = fb.width.saturating_sub(PANE_PAD_X);
    text::right(fb, right_x, top, state.sort.label(), ACCENT, BODY_PX);
}

fn pair(fb: &mut PaintBuffer, x: u32, top: u32, label: &[u8], value: &[u8]) -> u32 {
    let after = text::left(fb, x, top, label, MUTED, BODY_PX).max(0) as u32 + STATUS_GAP;
    text::mono(fb, after, top, value, TITLE, BODY_PX).max(0) as u32 + STATUS_GROUP_GAP
}

// No findings is itself a claim, so the strip states it rather than going blank.
fn posture(state: &State) -> (u32, &'static [u8]) {
    match state.alerts.iter().map(|a| a.level).max() {
        None => (OK, b"SECURE"),
        Some(Level::Info) => (OK, Level::Info.label()),
        Some(Level::Warn) => (AMBER, Level::Warn.label()),
        Some(Level::Critical) => (DANGER, Level::Critical.label()),
    }
}
