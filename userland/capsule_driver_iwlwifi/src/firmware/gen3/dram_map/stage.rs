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

//! Copy every firmware section into the DMA staging region and report each
//! chunk's page-aligned device address.

use alloc::vec::Vec;

use super::super::image::Section;
use super::super::prph_scratch::MAX_DRAM_ENTRY;
use super::align::align_up;
use super::classify::classify;
use super::markers::CHUNK_ALIGN;
use super::placement::DramPlacement;

/// Stage the firmware images into `user_va` (the same buffer the device sees at
/// `device_addr`) and return each chunk's device address. Fails if an image has
/// more than [`MAX_DRAM_ENTRY`] chunks or the region cannot hold the images.
///
/// # Safety
/// `user_va` must point to at least `capacity` writable bytes and `device_addr`
/// must be the physical address the device sees for that same region.
pub unsafe fn stage(
    blob: &[u8],
    user_va: u64,
    device_addr: u64,
    capacity: usize,
) -> Result<DramPlacement, &'static str> {
    let l = classify(blob);
    if l.lmac.len() > MAX_DRAM_ENTRY
        || l.umac.len() > MAX_DRAM_ENTRY
        || l.virt.len() > MAX_DRAM_ENTRY
    {
        return Err("iwlwifi: firmware image has more chunks than the DRAM map holds");
    }
    let mut cursor = 0u64;
    let mut place = |secs: &[Section]| -> Result<Vec<u64>, &'static str> {
        let mut addrs = Vec::with_capacity(secs.len());
        for s in secs {
            cursor = align_up(cursor, CHUNK_ALIGN);
            if cursor as usize + s.data.len() > capacity {
                return Err("iwlwifi: firmware images overrun the staging region");
            }
            core::ptr::copy_nonoverlapping(s.data.as_ptr(), (user_va + cursor) as *mut u8, s.data.len());
            addrs.push(device_addr + cursor);
            cursor += s.data.len() as u64;
        }
        Ok(addrs)
    };
    let lmac = place(&l.lmac)?;
    let umac = place(&l.umac)?;
    let virt = place(&l.virt)?;
    Ok(DramPlacement { lmac, umac, virt, staged_bytes: cursor as usize })
}
