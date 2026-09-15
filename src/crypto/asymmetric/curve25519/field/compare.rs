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

use super::types::FieldElement;
use core::sync::atomic::{compiler_fence, Ordering};

impl FieldElement {
    pub fn ct_eq(&self, other: &FieldElement) -> bool {
        let a = self.to_bytes();
        let b = other.to_bytes();
        let mut diff = 0u8;
        for i in 0..32 {
            diff |= a[i] ^ b[i];
        }
        diff == 0
    }

    pub fn is_negative(&self) -> bool {
        let bytes = self.to_bytes();
        (bytes[0] & 1) == 1
    }

    pub fn neg(&self) -> FieldElement {
        FieldElement::zero().sub(self)
    }

    pub fn conditional_swap(swap: u8, a: &mut FieldElement, b: &mut FieldElement) {
        let mask = (swap as u64).wrapping_neg();
        for i in 0..5 {
            let t = mask & (a.0[i] ^ b.0[i]);
            a.0[i] ^= t;
            b.0[i] ^= t;
        }
    }

    pub fn zeroize(&mut self) {
        for limb in &mut self.0 {
            // SAFETY: Volatile write prevents compiler optimization of zeroization.
            unsafe {
                core::ptr::write_volatile(limb, 0);
            }
        }
        compiler_fence(Ordering::SeqCst);
    }

    pub fn is_zero(&self) -> bool {
        let bytes = self.to_bytes();
        let mut acc = 0u8;
        for b in &bytes {
            acc |= *b;
        }
        acc == 0
    }

    pub fn equals(&self, other: &FieldElement) -> bool {
        let a = self.to_bytes();
        let b = other.to_bytes();
        let mut diff = 0u8;
        for i in 0..32 {
            diff |= a[i] ^ b[i];
        }
        diff == 0
    }
}
