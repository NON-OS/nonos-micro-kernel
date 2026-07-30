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

use super::constants::{ASSUMED_CPU_FREQ_MHZ, RATE_LIMIT_WINDOW_MS};
use super::error::DriverError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverOpType {
    IoCommand,
    AdminCommand,
    ControlOp,
    DmaSetup,
    InterruptHandler,
}

impl DriverOpType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DriverOpType::IoCommand => "I/O Command",
            DriverOpType::AdminCommand => "Admin Command",
            DriverOpType::ControlOp => "Control Op",
            DriverOpType::DmaSetup => "DMA Setup",
            DriverOpType::InterruptHandler => "Interrupt Handler",
        }
    }
}

pub struct RateLimiter {
    max_ops_per_sec: u32,
    current_count: AtomicU64,
    last_reset_ms: AtomicU64,
}

impl RateLimiter {
    pub const fn new(max_ops_per_sec: u32) -> Self {
        Self { max_ops_per_sec, current_count: AtomicU64::new(0), last_reset_ms: AtomicU64::new(0) }
    }

    pub fn check_rate(&self, _op_type: DriverOpType) -> Result<(), DriverError> {
        if self.max_ops_per_sec == 0 {
            return Ok(());
        }

        let current_time_ms = self.get_current_time_ms();
        let last_reset = self.last_reset_ms.load(Ordering::Relaxed);

        if current_time_ms.saturating_sub(last_reset) >= RATE_LIMIT_WINDOW_MS {
            let _ = self.last_reset_ms.compare_exchange(
                last_reset,
                current_time_ms,
                Ordering::AcqRel,
                Ordering::Relaxed,
            );
            self.current_count.store(0, Ordering::Release);
        }

        let count = self.current_count.fetch_add(1, Ordering::AcqRel);

        if count >= self.max_ops_per_sec as u64 {
            self.current_count.fetch_sub(1, Ordering::AcqRel);
            return Err(DriverError::RateLimitExceeded);
        }

        Ok(())
    }

    pub fn try_acquire(&self, op_type: DriverOpType) -> bool {
        self.check_rate(op_type).is_ok()
    }

    pub fn reset(&self) {
        self.current_count.store(0, Ordering::Release);
        self.last_reset_ms.store(self.get_current_time_ms(), Ordering::Release);
    }

    pub fn stats(&self) -> (u64, u32) {
        (self.current_count.load(Ordering::Acquire), self.max_ops_per_sec)
    }

    pub fn remaining(&self) -> u64 {
        if self.max_ops_per_sec == 0 {
            return u64::MAX;
        }

        let current = self.current_count.load(Ordering::Acquire);
        (self.max_ops_per_sec as u64).saturating_sub(current)
    }

    pub fn is_exhausted(&self) -> bool {
        if self.max_ops_per_sec == 0 {
            return false;
        }
        self.current_count.load(Ordering::Acquire) >= self.max_ops_per_sec as u64
    }

    pub fn set_limit(&mut self, max_ops_per_sec: u32) {
        self.max_ops_per_sec = max_ops_per_sec;
    }

    fn get_current_time_ms(&self) -> u64 {
        #[cfg(target_arch = "x86_64")]
        {
            // SAFETY: rdtsc is safe to call on x86_64.
            let tsc = crate::arch::read_time_counter();
            tsc / (ASSUMED_CPU_FREQ_MHZ * 1000)
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            COUNTER.fetch_add(1, Ordering::Relaxed) / 1000
        }
    }
}

pub struct MultiRateLimiter {
    io_limiter: RateLimiter,
    admin_limiter: RateLimiter,
    dma_limiter: RateLimiter,
}

impl MultiRateLimiter {
    pub const fn new(io_limit: u32, admin_limit: u32, dma_limit: u32) -> Self {
        Self {
            io_limiter: RateLimiter::new(io_limit),
            admin_limiter: RateLimiter::new(admin_limit),
            dma_limiter: RateLimiter::new(dma_limit),
        }
    }

    pub fn check_rate(&self, op_type: DriverOpType) -> Result<(), DriverError> {
        match op_type {
            DriverOpType::IoCommand => self.io_limiter.check_rate(op_type),
            DriverOpType::AdminCommand | DriverOpType::ControlOp => {
                self.admin_limiter.check_rate(op_type)
            }
            DriverOpType::DmaSetup => self.dma_limiter.check_rate(op_type),
            DriverOpType::InterruptHandler => Ok(()),
        }
    }

    pub fn reset_all(&self) {
        self.io_limiter.reset();
        self.admin_limiter.reset();
        self.dma_limiter.reset();
    }
}
