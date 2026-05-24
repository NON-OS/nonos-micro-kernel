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
use alloc::vec::Vec;
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
const SCRATCHPAD_PAGE_BYTES: u64 = 4096;
const SCRATCHPAD_PTR_BYTES: u64 = 8;
pub enum Scratchpads {
    None,
    Allocated { array: DmaRegion, pages: Vec<DmaRegion> },
}
impl Scratchpads {
    pub fn allocate(pool: &DmaPool, count: u32) -> XhciResult<Self> {
        if count == 0 {
            return Ok(Scratchpads::None);
        }
        let array_bytes = (count as u64) * SCRATCHPAD_PTR_BYTES;
        let array = pool.alloc(array_bytes)?;
        array.zero();
        let mut pages: Vec<DmaRegion> = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let page = pool.alloc(SCRATCHPAD_PAGE_BYTES)?;
            page.zero();
            pages.push(page);
        }
        let array_va = array.as_mut_ptr::<u64>();
        for (i, page) in pages.iter().enumerate() {
            unsafe {
                core::ptr::write_volatile(array_va.add(i), page.phys());
            }
        }
        Ok(Scratchpads::Allocated { array, pages })
    }
    pub fn array_phys(&self) -> u64 {
        match self {
            Scratchpads::None => 0,
            Scratchpads::Allocated { array, .. } => array.phys(),
        }
    }
    pub fn page_count(&self) -> u32 {
        match self {
            Scratchpads::None => 0,
            Scratchpads::Allocated { pages, .. } => pages.len() as u32,
        }
    }
}