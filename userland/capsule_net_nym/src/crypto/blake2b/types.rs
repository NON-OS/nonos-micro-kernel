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

pub(super) const BLOCK_BYTES: usize = 128;

pub struct Blake2b {
    pub(super) h: [u64; 8],
    pub(super) buf: [u8; BLOCK_BYTES],
    pub(super) buf_len: usize,
    pub(super) counter: u128,
    pub(super) out_len: usize,
}

impl Drop for Blake2b {
    fn drop(&mut self) {
        for word in self.h.iter_mut() {
            unsafe { core::ptr::write_volatile(word, 0) };
        }
        for byte in self.buf.iter_mut() {
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
