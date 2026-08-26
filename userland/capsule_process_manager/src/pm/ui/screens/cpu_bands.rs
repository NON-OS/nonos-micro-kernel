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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::pm::format::{pct_1dp, state_label, u32_decimal};
use crate::pm::state::{Row, State};

use super::super::bars;
use super::super::chrome::Rect;
use super::super::metrics::{BAR_ROW_H, PANEL_PAD};
use super::super::tint::state_tint;
use super::cpu::panel;
use super::ovw_cards::load_tint;

const TOP_N: usize = 8;

// The scheduler codes the kernel actually reports, running first. A state with
// no members still draws its empty track, so the rows hold their positions from
// one refresh to the next instead of shuffling under the eye.
const STATES: [u8; 5] = [2, 1, 3, 4, 5];

pub(super) fn states(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let mut y = panel(fb, r, b"SCHEDULER STATE");
    let total = state.rows.len() as u64;
    let w = r.w.saturating_sub(PANEL_PAD * 2);
    let x = r.x + PANEL_PAD;
    for code in STATES {
        let n = state.rows.iter().filter(|row| row.state == code).count() as u64;
        let mut buf = [0u8; 12];
        let len = u32_decimal(n as u32, &mut buf);
        let label = state_label(code);
        bars::labelled(fb, x, y, w, label, &buf[..len], n, total, state_tint(code));
        y += BAR_ROW_H;
    }
}

// Scaled to the busiest process rather than to 100, so the shape stays readable
// on an idle system where every share rounds to nothing.
pub(super) fn consumers(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let mut y = panel(fb, r, b"TOP CONSUMERS");
    let mut rows: Vec<&Row> = state.rows.iter().collect();
    rows.sort_unstable_by(|a, b| b.cpu_pct.cmp(&a.cpu_pct));
    let top = rows.first().map(|row| row.cpu_pct).unwrap_or(0).max(1) as u64;
    let w = r.w.saturating_sub(PANEL_PAD * 2);
    let x = r.x + PANEL_PAD;
    for row in rows.iter().take(TOP_N) {
        let mut buf = [0u8; 12];
        let len = pct_1dp(row.cpu_pct, &mut buf);
        let tint = load_tint(row.cpu_pct as u32);
        bars::labelled(fb, x, y, w, row.name(), &buf[..len], row.cpu_pct as u64, top, tint);
        y += BAR_ROW_H;
    }
}
