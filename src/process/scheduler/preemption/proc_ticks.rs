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

use core::sync::atomic::{AtomicU64, Ordering};

pub const PROC_TICK_SLOTS: usize = 256;

static PROC_TICKS: [AtomicU64; PROC_TICK_SLOTS] = [const { AtomicU64::new(0) }; PROC_TICK_SLOTS];

pub fn charge_tick(pid: u32) {
    if pid != 0 {
        PROC_TICKS[(pid as usize) % PROC_TICK_SLOTS].fetch_add(1, Ordering::Relaxed);
    }
}

pub fn ticks_for(pid: u32) -> u64 {
    PROC_TICKS[(pid as usize) % PROC_TICK_SLOTS].load(Ordering::Relaxed)
}

pub fn clear(pid: u32) {
    PROC_TICKS[(pid as usize) % PROC_TICK_SLOTS].store(0, Ordering::Relaxed);
}
