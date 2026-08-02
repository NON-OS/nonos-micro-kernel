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

// Pulls the packaged files off the block device. Payloads run to hundreds of
// kilobytes while the driver caps a request at MAX_READ_BYTES, so every extent
// is walked in whole-sector chunks and trimmed back to its declared length.
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::client::{capacity, read_blocks};
use super::error::BlkError;
use super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN};
use super::store_toc::decode;
use super::wire::{MAX_READ_BYTES, SECTOR_SIZE};

pub struct StoreEntry {
    pub name: String,
    pub data: Vec<u8>,
}

pub fn load() -> Result<Vec<StoreEntry>, BlkError> {
    let capacity_bytes = capacity()?
        .checked_mul(SECTOR_SIZE as u64)
        .ok_or(BlkError::BadLength)?;
    let mut head = [0u8; SECTOR_SIZE];
    read_blocks(0, &mut head)?;
    let count = entry_count(&head)?;
    let mut toc = vec![0u8; sector_span(HEADER_LEN + ENTRY_LEN * count)];
    read_blocks(0, &mut toc)?;
    let mut staged = Vec::with_capacity(count);
    for entry in decode(&toc, count, capacity_bytes)? {
        let data = read_extent(entry.offset, entry.len)?;
        staged.push(StoreEntry { name: entry.name, data });
    }
    Ok(staged)
}

fn read_extent(offset: u64, len: u64) -> Result<Vec<u8>, BlkError> {
    let mut data = Vec::with_capacity(len as usize);
    let mut lba = offset / SECTOR_SIZE as u64;
    let mut scratch = vec![0u8; MAX_READ_BYTES];
    while (data.len() as u64) < len {
        let want = core::cmp::min(len - data.len() as u64, MAX_READ_BYTES as u64) as usize;
        let chunk = sector_span(want);
        read_blocks(lba, &mut scratch[..chunk])?;
        data.extend_from_slice(&scratch[..want]);
        lba += (chunk / SECTOR_SIZE) as u64;
    }
    Ok(data)
}

fn sector_span(bytes: usize) -> usize {
    bytes.div_ceil(SECTOR_SIZE) * SECTOR_SIZE
}
