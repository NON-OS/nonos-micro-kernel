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

//! The kernel's own pace: load averages, then each per-second counter as
//! its last minute of shape with the current figure beside it. Every
//! trace is scaled to its own peak, so a quiet machine still shows one.

use nonos_app_skeleton::PaintBuffer;

use crate::pm::format_sys::rate_human;
use crate::pm::state::{RateRing, State};
use crate::pm::theme::{ACCENT, FOREGROUND};

use super::super::chrome::Rect;
use super::super::metrics::{BODY_PX, NUM_PX, PANEL_PAD, SPARK_H};
use super::super::{spark, text};
use super::cpu::panel;

const TRACE_ROW_H: u32 = SPARK_H + 10;
const GAP: u32 = 14;
const LABEL_W: u32 = 118;
const VALUE_W: u32 = 60;

pub(super) fn kernel(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let mut y = panel(fb, r, b"KERNEL");
    let w = r.w.saturating_sub(PANEL_PAD * 2);
    let x = r.x + PANEL_PAD;
    let right = x + w;
    let mut buf = [0u8; 40];
    y = super::cpu_kernel_head::head(state, fb, x, y, right);
    let h = &state.history;
    let rows: [(&[u8], &RateRing); 4] = [
        (b"syscalls / s", &h.syscalls),
        (b"messages / s", &h.messages),
        (b"switches / s", &h.switches),
        (b"interrupts / s", &h.interrupts),
    ];
    for (label, ring) in rows {
        let top = text::centred_top(y, TRACE_ROW_H, BODY_PX);
        text::left(fb, x, top, label, FOREGROUND, BODY_PX);
        let n = rate_human(ring.latest(), &mut buf);
        text::mono_right(fb, right, top, &buf[..n], ACCENT, NUM_PX);
        let sx = x + LABEL_W + GAP;
        let sw = right.saturating_sub(VALUE_W + GAP).saturating_sub(sx);
        spark::rate(
            fb,
            sx,
            y + (TRACE_ROW_H - SPARK_H) / 2,
            sw,
            SPARK_H,
            ring,
            ring.peak() + ring.peak() / 4 + 1,
            ACCENT,
        );
        y += TRACE_ROW_H;
    }
}
