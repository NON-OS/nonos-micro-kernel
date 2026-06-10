// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::jitter::add_random_delay;

pub struct TimingGuard {
    min_cycles: u64,
    start: u64,
}

impl TimingGuard {
    pub fn new(min_microseconds: u64) -> Self {
        Self { min_cycles: min_microseconds * 2000, start: read_tsc() }
    }
    fn elapsed(&self) -> u64 {
        read_tsc().saturating_sub(self.start)
    }
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        if elapsed < self.min_cycles {
            for _ in 0..(self.min_cycles - elapsed) {
                core::hint::spin_loop();
            }
        }
        add_random_delay();
    }
}

fn read_tsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}
