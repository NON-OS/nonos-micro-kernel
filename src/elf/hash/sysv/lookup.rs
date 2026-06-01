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

use super::{hash::sysv_hash, state::SysvHashTable};

impl SysvHashTable {
    pub fn lookup(&self, name: &str) -> Option<usize> {
        if self.header.nbuckets == 0 {
            return None;
        }
        let bucket_idx = usize::try_from(sysv_hash(name.as_bytes()) % self.header.nbuckets).ok()?;
        let bucket_ptr = (self.buckets.as_u64() + (bucket_idx * 4) as u64) as *const u32;
        let mut sym_idx = usize::try_from(unsafe { ptr::read(bucket_ptr) }).ok()?;
        let nchains = usize::try_from(self.header.nchains).ok()?;
        while sym_idx != 0 {
            if sym_idx >= nchains {
                return None;
            }
            if self.compare_symbol_name(sym_idx, name) {
                return Some(sym_idx);
            }
            let chain_ptr = (self.chains.as_u64() + (sym_idx * 4) as u64) as *const u32;
            sym_idx = usize::try_from(unsafe { ptr::read(chain_ptr) }).ok()?;
        }
        None
    }

    pub fn bucket_count(&self) -> u32 { self.header.nbuckets }
    pub fn chain_count(&self) -> u32 { self.header.nchains }
    pub fn symbol_count(&self) -> usize { usize::try_from(self.header.nchains).unwrap_or(0) }
}
