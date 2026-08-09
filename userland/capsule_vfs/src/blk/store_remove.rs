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

//! Rewrites the NONOSTR1 TOC without the named entry. Payload extents stay
//! where they are; only descriptors compact. Sector 0 commits last, so a torn
//! rewrite fails closed: decode rejects the stale count rather than load it.
use alloc::vec;
use alloc::vec::Vec;

use super::client::{capacity, read_blocks};
use super::error::BlkError;
use super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN};
use super::store_toc::decode;
use super::store_write::commit;
use super::wire::SECTOR_SIZE;

const STORE_BASE_LBA: u64 = 256;

pub fn remove(name: &str) -> Result<(), BlkError> {
    let mut head = [0u8; SECTOR_SIZE];
    read_blocks(STORE_BASE_LBA, &mut head)?;
    let count = entry_count(&head)?;
    let span = sector_span(HEADER_LEN + ENTRY_LEN * count);
    let mut toc = vec![0u8; span];
    read_blocks(STORE_BASE_LBA, &mut toc)?;
    let capacity_bytes = capacity()?
        .checked_mul(SECTOR_SIZE as u64)
        .ok_or(BlkError::BadLength)?;
    let entries = decode(&toc, count, capacity_bytes)?;
    let keep: Vec<usize> = (0..count).filter(|&i| entries[i].name != name).collect();
    if keep.len() == count {
        return Ok(());
    }
    let mut region = vec![0u8; span];
    region[..HEADER_LEN].copy_from_slice(&toc[..HEADER_LEN]);
    for (slot, &src) in keep.iter().enumerate() {
        let from = HEADER_LEN + ENTRY_LEN * src;
        let to = HEADER_LEN + ENTRY_LEN * slot;
        region[to..to + ENTRY_LEN].copy_from_slice(&toc[from..from + ENTRY_LEN]);
    }
    region[12..16].copy_from_slice(&(keep.len() as u32).to_le_bytes());
    commit(&region)
}

fn sector_span(bytes: usize) -> usize {
    bytes.div_ceil(SECTOR_SIZE) * SECTOR_SIZE
}
