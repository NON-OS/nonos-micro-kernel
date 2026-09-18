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

//! The processor and memory cards. The processor figure is the share of
//! ticks the kernel did not spend halted, so an idle machine reads near
//! zero; memory is what the frame allocator has handed out against what
//! the boot map gave it.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;

use crate::pm::format::{mem_human, pct_1dp};
use crate::pm::state::State;

use super::super::card;
use super::ovw_cards::{load_tint, meter, sub_n};

pub(super) fn cpu(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let pct = state.sys.busy_pct.min(100);
    let mut buf = [0u8; 24];
    let n = pct_1dp(pct, &mut buf);
    let mut sub = [0u8; 48];
    let mut s = sub_n(&mut sub, b"user ", state.sys.user_pct as u32, b"%");
    s = sub_n(&mut sub[s..], b"  sys ", state.sys.kernel_pct as u32, b"%") + s;
    s = sub_n(&mut sub[s..], b"  idle ", state.sys.idle_pct as u32, b"%") + s;
    let band = card::paint(fb, x, y, w, IconId::PmCpu, b"PROCESSOR", &buf[..n], b"busy", &sub[..s]);
    meter(fb, (x, y, w), band, pct as u64, 100, load_tint(pct as u32));
}

pub(super) fn memory(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let (used, total) = (state.sys.mem_used_kb(), state.sys.mem_total_kb.max(1));
    let mut buf = [0u8; 24];
    let n = mem_human(used, &mut buf);
    let mut sub = [0u8; 32];
    let mut s = b"of ".len();
    sub[..s].copy_from_slice(b"of ");
    s += mem_human(total, &mut sub[s..]);
    let pct = (used.saturating_mul(100) / total).min(100) as u32;
    let band = card::paint(fb, x, y, w, IconId::PmMemory, b"MEMORY", &buf[..n], b"used", &sub[..s]);
    meter(fb, (x, y, w), band, used, total, load_tint(pct));
}
