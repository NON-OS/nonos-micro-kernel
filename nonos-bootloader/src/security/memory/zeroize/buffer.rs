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

use super::core::zeroize_slice;

pub struct SecureBuffer<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> SecureBuffer<N> {
    pub const fn new() -> Self {
        Self { data: [0u8; N] }
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn copy_from(&mut self, src: &[u8]) {
        let len = src.len().min(N);
        self.data[..len].copy_from_slice(&src[..len]);
    }
}

impl<const N: usize> Drop for SecureBuffer<N> {
    fn drop(&mut self) {
        zeroize_slice(&mut self.data);
    }
}
