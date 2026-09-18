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

use crate::pm::format::{mem_human, pct_1dp};
use crate::pm::format_labels::share_pct;
use crate::pm::state::{Row, State};
use crate::pm::theme::{CARD_BG, CARD_BORDER, FOREGROUND, LABEL, MUTED, TITLE};

use super::super::metrics::{
    BAR_H, BAR_ROW_H, BODY_PX, NUM_PX, PANEL_HEAD_H, PANEL_PAD, PANEL_RADIUS,
};
use super::super::tint::state_tint;
use super::super::{bars, text};

const NAME_W: u32 = 214;
const VALUE_W: u32 = 96;
const BAR_GAP: u32 = 14;

// Bars scale to the largest process; the share of installed RAM is printed.
pub fn paint(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_round(x, y, w, h, PANEL_RADIUS, CARD_BG);
    fb.stroke_round(x, y, w, h, PANEL_RADIUS, 1, CARD_BORDER);
    let top = text::centred_top(y, PANEL_HEAD_H, BODY_PX);
    text::left(fb, x + PANEL_PAD, top, b"TOP CONSUMERS", TITLE, BODY_PX);
    let right = x + w.saturating_sub(PANEL_PAD);
    text::right(fb, right, top, b"share of RAM, mmapped, resident", MUTED, BODY_PX);
    let mut rows = state.filtered();
    rows.sort_unstable_by(|a, b| b.mem_kb.cmp(&a.mem_kb));
    let den = rows.first().map(|row| row.mem_kb).unwrap_or(0).max(1);
    let slots = (h.saturating_sub(PANEL_HEAD_H + PANEL_PAD) / BAR_ROW_H) as usize;
    for (slot, row) in rows.iter().take(slots).enumerate() {
        entry(
            fb,
            x,
            y + PANEL_HEAD_H + slot as u32 * BAR_ROW_H,
            w,
            (row, den),
            state.sys.mem_total_kb,
        );
    }
}

fn entry(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, row: (&Row, u64), ram_kb: u64) {
    let (row, den) = row;
    let top = text::centred_top(y, BAR_ROW_H, BODY_PX);
    let name = text::fit(fb, row.name(), BODY_PX, NAME_W);
    text::left(fb, x + PANEL_PAD, top, name, FOREGROUND, BODY_PX);
    let mut buf = [0u8; 20];
    let n = mem_human(row.mem_kb, &mut buf);
    let right = x + w.saturating_sub(PANEL_PAD);
    text::mono_right(fb, right, top, &buf[..n], LABEL, NUM_PX);
    let n = mem_human(row.mapped_kb, &mut buf);
    text::mono_right(fb, right.saturating_sub(VALUE_W), top, &buf[..n], MUTED, NUM_PX);
    let n = pct_1dp(share_pct(row.mem_kb, ram_kb), &mut buf);
    text::mono_right(fb, right.saturating_sub(VALUE_W * 2), top, &buf[..n], MUTED, NUM_PX);
    let bx = x + PANEL_PAD + NAME_W + BAR_GAP;
    let bw = right.saturating_sub(VALUE_W * 3 + BAR_GAP).saturating_sub(bx);
    let by = y + (BAR_ROW_H - BAR_H) / 2;
    bars::hbar(fb, bx, by, bw, BAR_H, row.mem_kb, den, state_tint(row.state));
}
