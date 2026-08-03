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

pub const BLOCK_BYTES: usize = 16;
pub const KEY_BYTES: usize = 16;
pub(super) const ROUNDS: usize = 10;
pub(super) const EXPANDED_WORDS: usize = 4 * (ROUNDS + 1);

pub struct Aes128 {
    pub(super) round_keys: [u32; EXPANDED_WORDS],
}

impl Drop for Aes128 {
    fn drop(&mut self) {
        for word in self.round_keys.iter_mut() {
            unsafe { core::ptr::write_volatile(word, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
