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

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub struct CpuRunQueueStats {
    pub enqueued: AtomicU64,
    pub dequeued: AtomicU64,
    pub migrations_in: AtomicU64,
    pub migrations_out: AtomicU64,
    pub steals: AtomicU64,
}

impl CpuRunQueueStats {
    pub const fn new() -> Self {
        Self {
            enqueued: AtomicU64::new(0),
            dequeued: AtomicU64::new(0),
            migrations_in: AtomicU64::new(0),
            migrations_out: AtomicU64::new(0),
            steals: AtomicU64::new(0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CpuLoad {
    pub cpu_id: usize,
    pub queue_len: usize,
    pub last_tick: u64,
}

pub struct LoadBalanceState {
    pub last_balance_tick: AtomicU64,
    pub active_cpus: AtomicU32,
}

impl LoadBalanceState {
    pub const fn new() -> Self {
        Self { last_balance_tick: AtomicU64::new(0), active_cpus: AtomicU32::new(1) }
    }

    pub fn should_balance(&self, current_tick: u64, interval: u64) -> bool {
        let last = self.last_balance_tick.load(Ordering::Relaxed);
        super::interval::elapsed_reached(current_tick, last, interval)
    }

    pub fn mark_balanced(&self, tick: u64) {
        self.last_balance_tick.store(tick, Ordering::Relaxed);
    }
}
