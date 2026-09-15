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

#[inline]
pub(crate) fn ct_lt_u64(a: u64, b: u64) -> u64 {
    let x = a ^ ((a ^ b) | (a.wrapping_sub(b) ^ b));
    x >> 63
}

#[derive(Clone, PartialEq, Eq)]
pub struct FieldElement(pub(crate) [u64; 4]);

impl Drop for FieldElement {
    fn drop(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.0[0], 0);
            core::ptr::write_volatile(&mut self.0[1], 0);
            core::ptr::write_volatile(&mut self.0[2], 0);
            core::ptr::write_volatile(&mut self.0[3], 0);
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

impl FieldElement {
    pub const ZERO: Self = Self([0, 0, 0, 0]);
    pub const ONE: Self = Self([1, 0, 0, 0]);

    pub(crate) const P: [u64; 4] =
        [0xFFFFFFFEFFFFFC2F, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
}
