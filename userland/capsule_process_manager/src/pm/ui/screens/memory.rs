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

use crate::pm::format::{mem_human, u32_decimal};
use crate::pm::state::State;

use super::super::card;
use super::super::chrome::Rect;
use super::super::metrics::{CARD_GAP, CARD_H};
use super::{mem_consumers, mem_trend};

const CARDS: u32 = 3;
const BAND_GAP: u32 = 14;
const TREND_H: u32 = 96;

// Resident footprint, three ways: the totals as cards, the direction of travel as
// a sparkline, then who is actually holding it. There is no host total to divide
// by, so every share on this screen is a share of the resident sum and says so.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let w = r.w.saturating_sub(CARD_GAP * (CARDS - 1)) / CARDS;
    let mut buf = [0u8; 20];
    let n = mem_human(state.total_mem_kb, &mut buf);
    card::paint(fb, r.x, r.y, w, IconId::PmMemory, b"RESIDENT", &buf[..n], b"", b"summed");
    let n = u32_decimal(state.rows.len() as u32, &mut buf);
    let x = r.x + w + CARD_GAP;
    card::paint(fb, x, r.y, w, IconId::Processes, b"TRACKED", &buf[..n], b"procs", b"live");
    largest(state, fb, r.x + (w + CARD_GAP) * 2, r.y, w);
    let trend_y = r.y + CARD_H + BAND_GAP;
    mem_trend::paint(state, fb, r.x, trend_y, r.w, TREND_H);
    let list_y = trend_y + TREND_H + BAND_GAP;
    let list_h = r.h.saturating_sub(list_y - r.y);
    mem_consumers::paint(state, fb, r.x, list_y, r.w, list_h);
}

// The single biggest consumer, named rather than ranked: the number alone never
// answers the question the user actually has.
fn largest(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let top = state.rows.iter().max_by_key(|row| row.mem_kb);
    let mut buf = [0u8; 20];
    let n = match top {
        Some(row) => mem_human(row.mem_kb, &mut buf),
        None => 0,
    };
    let name = top.map(|row| row.name()).unwrap_or(b"-");
    card::paint(fb, x, y, w, IconId::FsFile, b"LARGEST", &buf[..n], b"", name);
}
