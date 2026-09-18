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

//! The inspector's label/value lines. Identity and scheduling here, the
//! kernel's counters in `insp_fields_more.rs`.

use nonos_app_skeleton::PaintBuffer;

use crate::pm::format::{mem_human, pct_1dp, state_label, u32_decimal, uptime_human};
use crate::pm::format_labels::{priority_label, share_pct};
use crate::pm::state::Row;
use crate::pm::theme::{FOREGROUND, MUTED, TITLE};

use super::insp_geom::content_w;
use super::metrics::{BODY_PX, INSP_FIELD_H};
use super::text;
use super::tint::state_tint;

/// One label/value line: the label left in MUTED, the value placed from the
/// pane's right edge in mono so the values form a column. Returns the y the
/// next line starts at.
pub fn field(fb: &mut PaintBuffer, x: u32, y: u32, label: &[u8], value: &[u8], tint: u32) -> u32 {
    let top = text::centred_top(y, INSP_FIELD_H, BODY_PX);
    text::left(fb, x, top, label, MUTED, BODY_PX);
    text::mono_right(fb, x + content_w(), top, value, tint, BODY_PX);
    y + INSP_FIELD_H
}

/// Who and how it is scheduled, then what it holds; `ram_kb` is installed
/// memory, so the share is of the machine and not of the table.
pub fn block(fb: &mut PaintBuffer, x: u32, y: u32, row: &Row, ram_kb: u64) -> u32 {
    let mut buf = [0u8; 24];
    let n = u32_decimal(row.pid, &mut buf);
    let mut y = field(fb, x, y, b"PID", &buf[..n], TITLE);
    let n = u32_decimal(row.ppid, &mut buf);
    y = field(fb, x, y, b"Parent", &buf[..n], FOREGROUND);
    y = field(fb, x, y, b"State", state_label(row.state), state_tint(row.state));
    y = field(fb, x, y, b"Priority", priority_label(row.priority), FOREGROUND);
    let n = uptime_human(row.uptime_ms / 1000, &mut buf);
    y = field(fb, x, y, b"Uptime", &buf[..n], FOREGROUND);
    let n = pct_1dp(row.cpu_pct, &mut buf);
    y = field(fb, x, y, b"CPU", &buf[..n], FOREGROUND);
    let n = mem_human(row.mem_kb, &mut buf);
    y = field(fb, x, y, b"Resident", &buf[..n], FOREGROUND);
    let n = pct_1dp(share_pct(row.mem_kb, ram_kb), &mut buf);
    y = field(fb, x, y, b"Share of RAM", &buf[..n], FOREGROUND);
    super::insp_fields_more::block(fb, x, y, row)
}
