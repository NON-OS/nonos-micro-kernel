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
use nonos_toolkit::icons::IconId;

use crate::pm::format::{mem_human, pct_1dp, u32_decimal};
use crate::pm::state::State;
use crate::pm::theme::{ACCENT, AMBER, DANGER};

use super::super::bars;
use super::super::card;
use super::super::metrics::{BAR_H, CARD_H, CARD_PAD};

// The card's (x, y, w) travels as one tuple so the meter fits one line. The band
// card::paint hands back is 26 px, and 13 once the card keeps its own bottom pad,
// so SPARK_H cannot fit there and a 13 px waveform would read as noise.
pub(super) fn meter(fb: &mut PaintBuffer, at: (u32, u32, u32), band: u32, n: u64, d: u64, t: u32) {
    let bar_y = (at.1 + CARD_H).saturating_sub(CARD_PAD + BAR_H).max(band);
    let w = at.2.saturating_sub(CARD_PAD * 2);
    bars::hbar(fb, at.0 + CARD_PAD, bar_y, w, BAR_H, n, d, t);
}

pub(super) fn load_tint(pct: u32) -> u32 {
    match pct {
        0..=20 => ACCENT,
        21..=50 => AMBER,
        _ => DANGER,
    }
}

// A subcaption of the form "peak 87%", built in place: there is no formatter.
pub(super) fn sub_n(out: &mut [u8], head: &[u8], v: u32, tail: &[u8]) -> usize {
    let mut i = head.len().min(out.len());
    out[..i].copy_from_slice(&head[..i]);
    i += u32_decimal(v, &mut out[i..]);
    let end = (i + tail.len()).min(out.len());
    out[i..end].copy_from_slice(&tail[..end - i]);
    end
}

pub(super) fn cpu(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let pct = state.total_cpu.min(100);
    let mut buf = [0u8; 24];
    let n = pct_1dp(pct as u8, &mut buf);
    let mut sub = [0u8; 24];
    let s = sub_n(&mut sub, b"peak ", state.history.total.peak_cpu() as u32, b"%");
    let band = card::paint(fb, x, y, w, IconId::PmCpu, b"CPU LOAD", &buf[..n], b"", &sub[..s]);
    meter(fb, (x, y, w), band, pct as u64, 100, load_tint(pct));
}

// Resident memory has no host total to divide by, so the meter reads against the
// tallest sample the window holds, which is the same honesty spark::mem keeps.
pub(super) fn memory(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let mut buf = [0u8; 24];
    let n = mem_human(state.total_mem_kb, &mut buf);
    let ring = &state.history.total;
    let peak = (0..ring.len()).map(|i| ring.mem_at(i)).max().unwrap_or(0);
    let icon = IconId::PmMemory;
    let band = card::paint(fb, x, y, w, icon, b"MEMORY", &buf[..n], b"", b"resident sum");
    meter(fb, (x, y, w), band, state.total_mem_kb, peak as u64, ACCENT);
}
