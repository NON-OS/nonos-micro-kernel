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

#[derive(Debug, Clone)]
pub struct UefiStats {
    pub total_variables: u64,
    pub variable_reads: u64,
    pub variable_writes: u64,
    pub variable_read_errors: u64,
    pub variable_write_errors: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub secure_boot_enabled: bool,
    pub setup_mode: bool,
    pub runtime_services_available: bool,
}

impl UefiStats {
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            (self.cache_hits as f64 / total as f64) * 100.0
        }
    }

    pub fn error_rate(&self) -> f64 {
        let total_ops = self.variable_reads + self.variable_writes;
        let total_errors = self.variable_read_errors + self.variable_write_errors;
        if total_ops == 0 {
            0.0
        } else {
            (total_errors as f64 / total_ops as f64) * 100.0
        }
    }
}

impl Default for UefiStats {
    fn default() -> Self {
        Self {
            total_variables: 0,
            variable_reads: 0,
            variable_writes: 0,
            variable_read_errors: 0,
            variable_write_errors: 0,
            cache_hits: 0,
            cache_misses: 0,
            secure_boot_enabled: false,
            setup_mode: true,
            runtime_services_available: false,
        }
    }
}

pub struct InternalStats {
    pub variable_reads: AtomicU64,
    pub variable_writes: AtomicU64,
    pub variable_read_errors: AtomicU64,
    pub variable_write_errors: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
}

impl InternalStats {
    pub const fn new() -> Self {
        Self {
            variable_reads: AtomicU64::new(0),
            variable_writes: AtomicU64::new(0),
            variable_read_errors: AtomicU64::new(0),
            variable_write_errors: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
        }
    }

    #[inline]
    pub fn inc_reads(&self) {
        self.variable_reads.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn inc_writes(&self) {
        self.variable_writes.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn inc_read_errors(&self) {
        self.variable_read_errors.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn inc_write_errors(&self) {
        self.variable_write_errors.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn inc_cache_hits(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn inc_cache_misses(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn reads(&self) -> u64 {
        self.variable_reads.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn writes(&self) -> u64 {
        self.variable_writes.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn read_errors(&self) -> u64 {
        self.variable_read_errors.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn write_errors(&self) -> u64 {
        self.variable_write_errors.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn cache_hits(&self) -> u64 {
        self.cache_hits.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn cache_misses(&self) -> u64 {
        self.cache_misses.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.variable_reads.store(0, Ordering::Relaxed);
        self.variable_writes.store(0, Ordering::Relaxed);
        self.variable_read_errors.store(0, Ordering::Relaxed);
        self.variable_write_errors.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
    }
}

impl Default for InternalStats {
    fn default() -> Self {
        Self::new()
    }
}
