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

use super::{hash::gnu_hash, state::GnuHashTable};

impl GnuHashTable {
    pub fn lookup(&self, name: &str) -> Option<usize> {
        if self.header.nbuckets == 0 {
            return None;
        }
        let hash = gnu_hash(name.as_bytes());
        if !self.check_bloom_filter(hash) {
            return None;
        }
        let bucket_idx = usize::try_from(hash % self.header.nbuckets).ok()?;
        let bucket_ptr = (self.buckets.as_u64() + (bucket_idx * 4) as u64) as *const u32;
        let sym_idx = usize::try_from(unsafe { ptr::read(bucket_ptr) }).ok()?;
        let sym_offset = usize::try_from(self.header.symoffset).ok()?;
        if sym_idx == 0 || sym_idx < sym_offset {
            return None;
        }
        let mut current_idx = sym_idx;
        let mut chain_pos = sym_idx - sym_offset;
        loop {
            let chain_ptr = (self.chains.as_u64() + (chain_pos * 4) as u64) as *const u32;
            let chain_entry = unsafe { ptr::read(chain_ptr) };
            if (chain_entry | 1) == (hash | 1) && self.compare_symbol_name(current_idx, name) {
                return Some(current_idx);
            }
            if chain_entry & 1 != 0 {
                return None;
            }
            current_idx += 1;
            chain_pos += 1;
        }
    }

    pub fn bucket_count(&self) -> u32 {
        self.header.nbuckets
    }
    pub fn sym_offset(&self) -> u32 {
        self.header.symoffset
    }
}
