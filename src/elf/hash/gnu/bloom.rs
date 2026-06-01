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

use core::ptr;

use super::state::GnuHashTable;

impl GnuHashTable {
    pub(super) fn check_bloom_filter(&self, hash: u32) -> bool {
        let bloom_size = self.header.bloom_size as u64;
        if bloom_size == 0 {
            return true;
        }
        let Ok(word_idx) = usize::try_from((u64::from(hash) / 64) % bloom_size) else {
            return false;
        };
        let bit1 = 1u64 << (hash % 64);
        let bit2 = 1u64 << ((hash >> self.header.bloom_shift) % 64);
        let word_ptr = (self.bloom_filter.as_u64() + (word_idx * 8) as u64) as *const u64;
        let bloom_word = unsafe { ptr::read(word_ptr) };
        (bloom_word & bit1 != 0) && (bloom_word & bit2 != 0)
    }
}
