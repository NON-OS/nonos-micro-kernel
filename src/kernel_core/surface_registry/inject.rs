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

use core::sync::atomic::{AtomicUsize, Ordering};

use super::input_ring::{consumer_ready, post_input};
use super::types::InputEvent;

const SCRIPT: &[u8] = b"NONOS";
const TICKS_PER_CHAR: usize = 30;
const GRAB_MARGIN_TICKS: usize = 200;
const UNARMED: usize = usize::MAX;
static IDX: AtomicUsize = AtomicUsize::new(0);
static ARMED_AT: AtomicUsize = AtomicUsize::new(UNARMED);
static TICK: AtomicUsize = AtomicUsize::new(0);

pub fn on_tick() {
    let t = TICK.fetch_add(1, Ordering::Relaxed);
    if !consumer_ready() {
        return;
    }
    let _ = ARMED_AT.compare_exchange(UNARMED, t, Ordering::AcqRel, Ordering::Relaxed);
    let armed = ARMED_AT.load(Ordering::Acquire);
    let elapsed = t.wrapping_sub(armed);
    if elapsed < GRAB_MARGIN_TICKS {
        return;
    }
    let phase = elapsed - GRAB_MARGIN_TICKS;
    if phase % TICKS_PER_CHAR != 0 {
        return;
    }
    let i = IDX.fetch_add(1, Ordering::Relaxed);
    if i >= SCRIPT.len() {
        return;
    }
    let ev = InputEvent {
        kind: 0,
        flags: 0,
        code: SCRIPT[i] as u32,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    let _ = post_input(ev);
}
