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

#[derive(Clone, PartialEq, Eq)]
pub struct Scalar(pub(crate) [u64; 4]);

impl Scalar {
    pub const ZERO: Self = Self([0, 0, 0, 0]);
    pub const ONE: Self = Self([1, 0, 0, 0]);

    pub(crate) const N: [u64; 4] =
        [0xBFD25E8CD0364141, 0xBAAEDCE6AF48A03B, 0xFFFFFFFFFFFFFFFE, 0xFFFFFFFFFFFFFFFF];
}

impl Drop for Scalar {
    fn drop(&mut self) {
        // SAFETY: volatile writes ensure zeroization isn't optimized out
        unsafe {
            core::ptr::write_volatile(&mut self.0[0], 0);
            core::ptr::write_volatile(&mut self.0[1], 0);
            core::ptr::write_volatile(&mut self.0[2], 0);
            core::ptr::write_volatile(&mut self.0[3], 0);
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
