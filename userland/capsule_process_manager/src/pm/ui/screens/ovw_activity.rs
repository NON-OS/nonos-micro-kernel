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

//! What the kernel is doing per second: syscalls, messages, context
//! switches and interrupts, each a difference of two kernel counters over
//! the seconds between them.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::icons::IconId;

use crate::pm::format_sys::rate_human;
use crate::pm::state::State;
use crate::pm::theme::{MUTED, TITLE};

use super::super::card;
use super::super::metrics::{BODY_PX, CARD_PAD};
use super::super::text;

pub(super) fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let mut row_y = card::head(fb, x, y, w, IconId::Processes, b"ACTIVITY / S");
    let line = line_height(BODY_PX).max(1) as u32 + 2;
    let (left, right) = (x + CARD_PAD, x + w.saturating_sub(CARD_PAD));
    let rows: [(&[u8], u32); 4] = [
        (b"syscalls", state.sys.sysc_ps),
        (b"messages", state.sys.ipc_ps),
        (b"switches", state.sys.ctx_ps),
        (b"interrupts", state.sys.irq_ps),
    ];
    for (label, value) in rows {
        text::left(fb, left, row_y, label, MUTED, BODY_PX);
        let mut buf = [0u8; 16];
        let n = rate_human(value, &mut buf);
        text::mono_right(fb, right, row_y, &buf[..n], TITLE, BODY_PX);
        row_y += line;
    }
}
