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

pub struct SmmStats {
    pub smi_count: AtomicU64,
    pub sw_smi_count: AtomicU64,
    pub timer_smi_count: AtomicU64,
    pub io_trap_smi_count: AtomicU64,
    pub integrity_checks: AtomicU64,
    pub integrity_failures: AtomicU64,
    pub handlers_verified: AtomicU64,
    pub regions_protected: AtomicU64,
}

impl SmmStats {
    pub const fn new() -> Self {
        Self {
            smi_count: AtomicU64::new(0),
            sw_smi_count: AtomicU64::new(0),
            timer_smi_count: AtomicU64::new(0),
            io_trap_smi_count: AtomicU64::new(0),
            integrity_checks: AtomicU64::new(0),
            integrity_failures: AtomicU64::new(0),
            handlers_verified: AtomicU64::new(0),
            regions_protected: AtomicU64::new(0),
        }
    }

    pub fn reset(&self) {
        self.smi_count.store(0, Ordering::SeqCst);
        self.sw_smi_count.store(0, Ordering::SeqCst);
        self.timer_smi_count.store(0, Ordering::SeqCst);
        self.io_trap_smi_count.store(0, Ordering::SeqCst);
        self.integrity_checks.store(0, Ordering::SeqCst);
        self.integrity_failures.store(0, Ordering::SeqCst);
        self.handlers_verified.store(0, Ordering::SeqCst);
        self.regions_protected.store(0, Ordering::SeqCst);
    }
}
