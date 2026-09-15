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

use super::super::P384_P;

#[derive(Clone, PartialEq, Eq)]
pub struct FieldElement(pub(crate) [u64; 6]);

impl Drop for FieldElement {
    fn drop(&mut self) {
        // SAFETY: Volatile writes prevent compiler optimization of zeroization.
        unsafe {
            for limb in self.0.iter_mut() {
                core::ptr::write_volatile(limb, 0);
            }
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

impl FieldElement {
    pub const ZERO: Self = Self([0, 0, 0, 0, 0, 0]);
    pub const ONE: Self = Self([1, 0, 0, 0, 0, 0]);
    pub(crate) const P: [u64; 6] = P384_P;

}
