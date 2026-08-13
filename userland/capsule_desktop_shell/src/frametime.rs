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

//! A feature-gated repaint timer. It compiles to nothing without `shell-frametime`,
//! so release layout is byte-identical and the layout-sensitive paths are untouched.
//!
//! Timing comes from the monotonic uptime clock, never the wall clock: the wall
//! clock is steppable via `mk_time_adjust`, which would let an adjustment land
//! mid-frame and produce a saturating-to-zero or wildly inflated sample.

#[cfg(feature = "shell-frametime")]
mod on {
    use core::sync::atomic::{AtomicU64, Ordering};

    use nonos_libc::debug::mk_debug;
    use nonos_libc::time::mk_uptime_ms;

    static FRAMES: AtomicU64 = AtomicU64::new(0);
    static TOTAL_MS: AtomicU64 = AtomicU64::new(0);
    const WINDOW: u64 = 60;

    pub fn begin() -> u64 {
        mk_uptime_ms() as u64
    }

    pub fn end(start: u64) {
        let elapsed = (mk_uptime_ms() as u64).saturating_sub(start);
        let total = TOTAL_MS.fetch_add(elapsed, Ordering::Relaxed) + elapsed;
        let n = FRAMES.fetch_add(1, Ordering::Relaxed) + 1;
        if n % WINDOW != 0 {
            return;
        }
        let mut line = *b"[SHELL-FT] frames=      avg_ms=      total=      \n";
        write_u64(&mut line[18..24], n);
        write_u64(&mut line[31..37], total / n);
        write_u64(&mut line[43..49], total);
        mk_debug(line.as_ptr(), line.len());
    }

    fn write_u64(field: &mut [u8], mut v: u64) {
        for slot in field.iter_mut().rev() {
            *slot = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
    }
}

#[cfg(not(feature = "shell-frametime"))]
mod on {
    pub fn begin() -> u64 {
        0
    }

    pub fn end(_start: u64) {}
}

pub use on::{begin, end};
