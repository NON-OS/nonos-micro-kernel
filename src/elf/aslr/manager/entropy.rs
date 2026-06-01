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

use core::arch::x86_64::_rdrand64_step;

use super::{constants::{FALLBACK_SEED, LCG_INCREMENT, LCG_MULTIPLIER}, state::AslrManager};

pub(super) fn gather_entropy() -> u64 {
    unsafe {
        let mut value = 0u64;
        if _rdrand64_step(&mut value) == 1 { value } else { FALLBACK_SEED }
    }
}

impl AslrManager {
    pub fn random_offset(&mut self, max_offset: u64) -> u64 {
        if max_offset == 0 { return 0; }
        let mut rand = 0u64;
        if unsafe { _rdrand64_step(&mut rand) } == 1 {
            self.entropy_pool ^= rand;
        } else {
            self.entropy_pool = self.entropy_pool.wrapping_mul(LCG_MULTIPLIER).wrapping_add(LCG_INCREMENT);
        }
        (self.entropy_pool >> 16) % max_offset
    }

    pub fn reseed(&mut self) { self.entropy_pool ^= gather_entropy(); }
    pub fn entropy(&self) -> u64 { self.entropy_pool }
}
