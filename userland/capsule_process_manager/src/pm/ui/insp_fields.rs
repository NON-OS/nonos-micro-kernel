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

use crate::pm::format::{mem_human, pct_1dp, state_label, u32_decimal, uptime_human};
use crate::pm::state::Row;
use crate::pm::theme::{FOREGROUND, MUTED, TITLE};

use super::insp_geom::content_w;
use super::metrics::{BODY_PX, INSP_FIELD_H};
use super::text;
use super::tint::state_tint;

// One label/value line: the label sits left in MUTED and the value is placed
// from the pane's right edge in mono, so the values form a column whatever the
// label runs to. Returns the y the next line starts at.
pub fn field(fb: &mut PaintBuffer, x: u32, y: u32, label: &[u8], value: &[u8], tint: u32) -> u32 {
    let top = text::centred_top(y, INSP_FIELD_H, BODY_PX);
    text::left(fb, x, top, label, MUTED, BODY_PX);
    text::mono_right(fb, x + content_w(), top, value, tint, BODY_PX);
    y + INSP_FIELD_H
}

// What the row carries, plus the share of total memory the table has no column
// for. Run ticks are not here: the kernel reports them, but Row does not keep
// the field and State::prev is private to the state layer.
pub fn block(fb: &mut PaintBuffer, x: u32, y: u32, row: &Row, total_kb: u64) -> u32 {
    let mut buf = [0u8; 24];
    let n = u32_decimal(row.pid, &mut buf);
    let mut y = field(fb, x, y, b"PID", &buf[..n], TITLE);
    y = field(fb, x, y, b"State", state_label(row.state), state_tint(row.state));
    let n = uptime_human(row.uptime_ms / 1000, &mut buf);
    y = field(fb, x, y, b"Uptime", &buf[..n], FOREGROUND);
    let n = pct_1dp(row.cpu_pct, &mut buf);
    y = field(fb, x, y, b"CPU", &buf[..n], FOREGROUND);
    let n = mem_human(row.mem_kb, &mut buf);
    y = field(fb, x, y, b"Resident", &buf[..n], FOREGROUND);
    let n = pct_1dp(share(row.mem_kb, total_kb), &mut buf);
    field(fb, x, y, b"Share of total", &buf[..n], FOREGROUND)
}

fn share(kb: u64, total_kb: u64) -> u8 {
    if total_kb == 0 {
        0
    } else {
        (kb.saturating_mul(100) / total_kb).min(100) as u8
    }
}
