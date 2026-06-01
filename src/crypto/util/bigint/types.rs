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

extern crate alloc;
use crate::crypto::constant_time::compiler_fence;
use alloc::vec::Vec;

pub const LIMB_BITS: usize = 64;

#[derive(Clone, Eq)]
pub struct BigUint {
    pub(crate) limbs: Vec<u64>,
}

impl Drop for BigUint {
    fn drop(&mut self) {
        for limb in &mut self.limbs {
            unsafe {
                core::ptr::write_volatile(limb, 0);
            }
        }
        compiler_fence();
    }
}

impl Default for BigUint {
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialEq for BigUint {
    fn eq(&self, other: &Self) -> bool {
        self.limbs == other.limbs
    }
}
