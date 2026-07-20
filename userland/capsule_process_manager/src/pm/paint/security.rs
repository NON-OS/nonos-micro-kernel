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

//! The security panel: a live attack-surface tally over the running set and the
//! findings the monitor derived from it, worst-first. Everything shown here is
//! computed from the same kernel process table the process view draws. Findings
//! are selectable; Enter jumps to the named process.

use nonos_app_skeleton::PaintBuffer;

use super::super::format::u32_decimal;
use super::super::layout::{SEC_LIST_TOP, SEC_ROW_H};
use super::super::security::{Level, Posture};
use super::super::state::State;
use super::super::theme::{
    ACCENT, AMBER, BAND, DANGER, FOREGROUND, HEADER_BG, MUTED, OK, RULE, SELECT_BG, TITLE, WARNING,
};
use super::{badge, t, BODY_PX, HEADER_H, HEADER_TEXT_Y, PAD, SMALL_PX};

const POSTURE_Y: u32 = 54;

// Tag colour for a finding by severity.
fn level_color(level: Level) -> u32 {
    match level {
        Level::Info => ACCENT,
        Level::Warn => AMBER,
        Level::Critical => DANGER,
    }
}

pub(super) fn paint(state: &mut State, fb: &mut PaintBuffer) {
    let footer_h = 34;
    let footer_y = fb.height.saturating_sub(footer_h);
    let visible = (footer_y.saturating_sub(SEC_LIST_TOP) / SEC_ROW_H).max(1) as usize;
    state.alert_visible = visible;
    if state.alert_scroll > state.alerts.len().saturating_sub(visible) {
        state.alert_scroll = state.alerts.len().saturating_sub(visible);
    }

    fb.fill_rect(0, 0, fb.width, HEADER_H, HEADER_BG);
    fb.fill_rect(0, HEADER_H, fb.width, 1, RULE);
    t(fb, PAD, HEADER_TEXT_Y, b"SECURITY", TITLE, BODY_PX);
    badge(fb, state);

    paint_posture(&state.monitor.posture, fb);
    paint_findings(state, fb, footer_y, visible);

    fb.fill_rect(0, footer_y, fb.width, 1, RULE);
    t(
        fb,
        PAD,
        footer_y + 9,
        b"up/down select   Enter jump to process   S/Esc back   R refresh",
        WARNING,
        SMALL_PX,
    );
}

// The honest attack-surface tally: how many live processes hold each sensitive
// authority. A count is a fact, not a verdict; raw-hardware and DMA reach are
// the sharpest because DMA bypasses the page tables entirely.
fn paint_posture(p: &Posture, fb: &mut PaintBuffer) {
    let cell = |fb: &mut PaintBuffer, x: u32, label: &[u8], value: u32, color: u32| {
        let mut buf = [0u8; 12];
        let n = u32_decimal(value, &mut buf);
        t(fb, x, POSTURE_Y, label, MUTED, SMALL_PX);
        t(fb, x, POSTURE_Y + 18, &buf[..n], color, BODY_PX);
    };
    cell(fb, PAD, b"processes", p.total, FOREGROUND);
    cell(fb, PAD + 150, b"admin", p.admin, if p.admin > 1 { AMBER } else { FOREGROUND });
    cell(fb, PAD + 290, b"raw hw", p.raw_hw, FOREGROUND);
    cell(fb, PAD + 430, b"dma", p.dma, if p.dma > 0 { AMBER } else { FOREGROUND });
    cell(fb, PAD + 560, b"spawn", p.spawn, FOREGROUND);
    cell(fb, PAD + 700, b"debug", p.debug, FOREGROUND);
    fb.fill_rect(PAD, POSTURE_Y + 34, fb.width.saturating_sub(2 * PAD), 1, RULE);
}

fn paint_findings(state: &State, fb: &mut PaintBuffer, footer_y: u32, visible: usize) {
    if state.alerts.is_empty() {
        t(fb, PAD, SEC_LIST_TOP, b"No findings.", OK, BODY_PX);
        t(
            fb,
            PAD,
            SEC_LIST_TOP + 24,
            b"All watched services are running, no process holds admin outside init,",
            MUTED,
            SMALL_PX,
        );
        t(fb, PAD, SEC_LIST_TOP + 44, b"and nothing is pinned at full cpu.", MUTED, SMALL_PX);
        return;
    }

    let end = (state.alert_scroll + visible).min(state.alerts.len());
    let mut y = SEC_LIST_TOP;
    for i in state.alert_scroll..end {
        let a = &state.alerts[i];
        if i == state.alert_sel {
            fb.fill_rect(0, y - 2, fb.width, SEC_ROW_H, SELECT_BG);
        }
        let color = level_color(a.level);
        fb.fill_rect(0, y - 2, 3, SEC_ROW_H, color);
        t(fb, PAD, y + 3, a.level.label(), color, SMALL_PX);

        let mut buf = [0u8; 12];
        if a.pid != 0 {
            let n = u32_decimal(a.pid, &mut buf);
            t(fb, PAD + 62, y + 3, &buf[..n], MUTED, SMALL_PX);
        } else {
            t(fb, PAD + 62, y + 3, b"-", MUTED, SMALL_PX);
        }

        let name = a.name();
        let name = if name.is_empty() { b"system".as_slice() } else { name };
        t(fb, PAD + 120, y + 3, name, TITLE, SMALL_PX);
        t(fb, PAD + 320, y + 3, a.msg, FOREGROUND, SMALL_PX);
        y += SEC_ROW_H;
    }

    // Proportional scrollbar when the findings overflow the panel.
    if state.alerts.len() > visible {
        let track_h = footer_y.saturating_sub(SEC_LIST_TOP);
        let x = fb.width.saturating_sub(6);
        fb.fill_rect(x, SEC_LIST_TOP, 4, track_h, BAND);
        let thumb_h = ((visible * track_h as usize) / state.alerts.len()).max(18) as u32;
        let span = track_h.saturating_sub(thumb_h);
        let max_scroll = state.alerts.len() - visible;
        let thumb_y = SEC_LIST_TOP + (state.alert_scroll as u32 * span) / max_scroll.max(1) as u32;
        fb.fill_rect(x, thumb_y, 4, thumb_h, MUTED);
    }
}
