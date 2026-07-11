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

//! The live process table: name, pid, scheduler state, cpu share, resident
//! memory and granted capabilities for the whole live set, with selection,
//! scrolling, totals and a decoded-capability line for the selected process.

use nonos_app_skeleton::PaintBuffer;

use super::super::critical::is_critical;
use super::super::format::{
    caps_summary, mem_human, state_label, u32_decimal, uptime_human, CAP_TABLE,
};
use super::super::layout::{COL_CAPS, COL_CPU, COL_MEM, COL_NAME, COL_PID, COL_STATE, COL_UPTIME};
use super::super::state::{Row, Sort, State};
use super::super::theme::{
    ACCENT, BAND, DANGER, FOREGROUND, HEADER_BG, MUTED, RULE, SELECT_BG, TITLE, WARNING,
};
use super::{
    badge, t, BODY_PX, FIRST_ROW_Y, HEADER_H, HEADER_TEXT_Y, PAD, ROW_H, ROW_TEXT_DY, SMALL_PX,
};

pub(super) fn paint(state: &mut State, fb: &mut PaintBuffer) {
    // Footer holds three lines: totals, capability detail, and status.
    let footer_h = 78;
    let footer_y = fb.height.saturating_sub(footer_h);
    let list_bottom = footer_y.saturating_sub(4);
    let visible = ((list_bottom.saturating_sub(FIRST_ROW_Y)) / ROW_H).max(1) as usize;
    state.visible = visible;
    if state.scroll > state.rows.len().saturating_sub(visible) {
        state.scroll = state.rows.len().saturating_sub(visible);
    }

    paint_header(state, fb);

    let mut y = FIRST_ROW_Y;
    let end = (state.scroll + visible).min(state.rows.len());
    for i in state.scroll..end {
        let row = &state.rows[i];
        let selected = row.pid == state.selected_pid && state.selected_pid != 0;
        if selected {
            fb.fill_rect(0, y - 2, fb.width, ROW_H, SELECT_BG);
        } else if i % 2 == 1 {
            fb.fill_rect(0, y - 2, fb.width, ROW_H, BAND);
        }
        // A thin stripe marks a protected core process: the monitor refuses to
        // end these, since killing one would strand the session.
        if is_critical(row.name()) {
            fb.fill_rect(0, y - 2, 3, ROW_H, WARNING);
        }
        paint_row(fb, y + ROW_TEXT_DY, row);
        y += ROW_H;
    }

    paint_scrollbar(fb, state, FIRST_ROW_Y, list_bottom, visible);
    paint_footer(state, fb, footer_y);
}

// A proportional scrollbar on the right edge, shown only when the list is taller
// than the window. Thumb size and position track the real scroll offset.
fn paint_scrollbar(fb: &mut PaintBuffer, state: &State, top: u32, bottom: u32, visible: usize) {
    let total = state.rows.len();
    if total <= visible || bottom <= top {
        return;
    }
    let track_h = bottom - top;
    let x = fb.width.saturating_sub(6);
    fb.fill_rect(x, top, 4, track_h, BAND);
    let thumb_h = ((visible * track_h as usize) / total).max(18) as u32;
    let span = track_h.saturating_sub(thumb_h);
    let max_scroll = total - visible;
    let thumb_y = top + (state.scroll as u32 * span) / max_scroll.max(1) as u32;
    fb.fill_rect(x, thumb_y, 4, thumb_h, MUTED);
}

fn paint_header(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, fb.width, HEADER_H, HEADER_BG);
    fb.fill_rect(0, HEADER_H, fb.width, 1, RULE);
    let col = |s: Sort| if s == state.sort { ACCENT } else { WARNING };
    t(fb, COL_NAME, HEADER_TEXT_Y, b"NAME", col(Sort::Name), BODY_PX);
    t(fb, COL_PID, HEADER_TEXT_Y, b"PID", col(Sort::Pid), BODY_PX);
    t(fb, COL_STATE, HEADER_TEXT_Y, b"STATE", WARNING, BODY_PX);
    t(fb, COL_CPU, HEADER_TEXT_Y, b"CPU", col(Sort::Cpu), BODY_PX);
    t(fb, COL_MEM, HEADER_TEXT_Y, b"MEMORY", col(Sort::Mem), BODY_PX);
    t(fb, COL_UPTIME, HEADER_TEXT_Y, b"UPTIME", WARNING, BODY_PX);
    t(fb, COL_CAPS, HEADER_TEXT_Y, b"CAPS", WARNING, BODY_PX);
    badge(fb, state);
}

fn paint_row(fb: &mut PaintBuffer, y: u32, row: &Row) {
    let mut buf = [0u8; 24];

    let name = row.name();
    let name = if name.is_empty() { b"?".as_slice() } else { name };
    t(fb, COL_NAME, y, name, FOREGROUND, BODY_PX);

    let n = u32_decimal(row.pid, &mut buf);
    t(fb, COL_PID, y, &buf[..n], MUTED, BODY_PX);

    let running = row.state == 2;
    t(fb, COL_STATE, y, state_label(row.state), if running { ACCENT } else { MUTED }, BODY_PX);

    let n = u32_decimal(row.cpu_pct as u32, &mut buf);
    t(fb, COL_CPU, y, &buf[..n], FOREGROUND, BODY_PX);
    let bar_x = COL_CPU + 42;
    let bar_w = 72u32;
    fb.fill_rect(bar_x, y + 3, bar_w, 7, BAND);
    let fill = (row.cpu_pct as u32 * bar_w / 100).min(bar_w);
    if fill > 0 {
        fb.fill_rect(bar_x, y + 3, fill, 7, ACCENT);
    }

    let n = mem_human(row.mem_kb, &mut buf);
    let mem_color = if row.mem_kb > 0 { FOREGROUND } else { MUTED };
    t(fb, COL_MEM, y, &buf[..n], mem_color, BODY_PX);

    // Kernel-computed uptime; grows every refresh, so the row reads as live.
    let n = uptime_human(row.uptime_ms / 1000, &mut buf);
    t(fb, COL_UPTIME, y, &buf[..n], FOREGROUND, BODY_PX);

    let n = caps_summary(row.caps, &mut buf);
    t(fb, COL_CAPS, y, &buf[..n], MUTED, BODY_PX);
}

fn paint_footer(state: &State, fb: &mut PaintBuffer, y: u32) {
    fb.fill_rect(0, y, fb.width, fb.height.saturating_sub(y), HEADER_BG);
    fb.fill_rect(0, y, fb.width, 1, RULE);
    let ty = y + 10;
    let mut buf = [0u8; 24];

    // Totals: process count, aggregate cpu, total resident memory, sort, keys.
    let n = u32_decimal(state.rows.len() as u32, &mut buf);
    t(fb, PAD, ty, b"procs", MUTED, SMALL_PX);
    t(fb, PAD + 62, ty, &buf[..n], TITLE, SMALL_PX);
    let n = u32_decimal(state.total_cpu, &mut buf);
    t(fb, PAD + 130, ty, b"cpu", MUTED, SMALL_PX);
    t(fb, PAD + 172, ty, &buf[..n], TITLE, SMALL_PX);
    let n = mem_human(state.total_mem_kb, &mut buf);
    t(fb, PAD + 240, ty, b"mem", MUTED, SMALL_PX);
    t(fb, PAD + 282, ty, &buf[..n], TITLE, SMALL_PX);
    t(fb, PAD + 400, ty, b"sort", MUTED, SMALL_PX);
    t(fb, PAD + 448, ty, state.sort.label(), ACCENT, SMALL_PX);
    let right = fb.width.saturating_sub(360);
    t(fb, right, ty, b"S security  K end  F force  R refresh", WARNING, SMALL_PX);

    // An armed kill takes over the detail line as a loud confirmation prompt.
    let dy = ty + 22;
    if state.pending_pid != 0 {
        t(fb, PAD, dy, state.notice, DANGER, SMALL_PX);
    } else if let Some(row) = state.rows.iter().find(|r| r.pid == state.selected_pid) {
        t(fb, PAD, dy, b"granted", MUTED, SMALL_PX);
        let mut x = PAD + 82;
        for &(bit, label) in CAP_TABLE {
            if row.caps & bit != 0 {
                t(fb, x, dy, label, ACCENT, SMALL_PX);
                x += label.len() as u32 * 9 + 12;
                if x > fb.width.saturating_sub(70) {
                    break;
                }
            }
        }
    } else {
        t(fb, PAD, dy, state.notice, MUTED, SMALL_PX);
    }

    t(fb, PAD, dy + 22, state.status, MUTED, SMALL_PX);
}
