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

//! RAM handed out, the largest run still free in one piece, the kernel's
//! own heap, what the processes have resident; then the trend and who.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;

use crate::pm::format::{mem_human, u64_decimal};
use crate::pm::format_sys::count_human;
use crate::pm::state::State;

use super::super::card;
use super::super::chrome::Rect;
use super::super::metrics::{CARD_GAP, CARD_H};
use super::{mem_consumers, mem_trend};

const CARDS: u32 = 4;
const BAND_GAP: u32 = 14;
const TREND_H: u32 = 96;

pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let w = r.w.saturating_sub(CARD_GAP * (CARDS - 1)) / CARDS;
    let step = w + CARD_GAP;
    let s = &state.sys;
    let mut buf = [0u8; 24];
    let mut sub = [0u8; 40];
    let n = mem_human(s.mem_used_kb(), &mut buf);
    let m = tail(&mut sub, b"of ", s.mem_total_kb, b"");
    card::paint(fb, r.x, r.y, w, IconId::PmMemory, b"RAM IN USE", &buf[..n], b"", &sub[..m]);
    let n = mem_human(s.largest_free_kb, &mut buf);
    let m = tail(&mut sub, b"free ", s.mem_free_kb, b" in all");
    let x = r.x + step;
    card::paint(fb, x, r.y, w, IconId::PmMemory, b"LARGEST FREE RUN", &buf[..n], b"", &sub[..m]);
    let n = mem_human(s.heap_used_kb, &mut buf);
    let m = tail(&mut sub, b"peak ", s.heap_peak_kb, b", ");
    let m = m + count_human(s.heap_allocs, &mut sub[m..]);
    let m = tail_bytes(&mut sub, m, b" allocs");
    let x = r.x + step * 2;
    card::paint(fb, x, r.y, w, IconId::FsFile, b"KERNEL HEAP", &buf[..n], b"", &sub[..m]);
    let n = mem_human(state.total_mem_kb, &mut buf);
    let m = u64_decimal(state.rows.len() as u64, &mut sub);
    let m = tail_bytes(&mut sub, m, b" processes resident");
    let x = r.x + step * 3;
    card::paint(fb, x, r.y, w, IconId::Processes, b"RESIDENT", &buf[..n], b"", &sub[..m]);
    let trend_y = r.y + CARD_H + BAND_GAP;
    mem_trend::paint(state, fb, r.x, trend_y, r.w, TREND_H);
    let list_y = trend_y + TREND_H + BAND_GAP;
    let list_h = r.h.saturating_sub(list_y - r.y);
    mem_consumers::paint(state, fb, r.x, list_y, r.w, list_h);
}

fn tail(out: &mut [u8], head: &[u8], kb: u64, rest: &[u8]) -> usize {
    let n = tail_bytes(out, 0, head);
    let n = n + mem_human(kb, &mut out[n..]);
    tail_bytes(out, n, rest)
}

fn tail_bytes(out: &mut [u8], at: usize, bytes: &[u8]) -> usize {
    let end = (at + bytes.len()).min(out.len());
    out[at..end].copy_from_slice(&bytes[..end - at]);
    end
}
