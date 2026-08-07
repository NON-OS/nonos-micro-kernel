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

use nonos_libc::mk_debug;

use super::client::{capacity, read_blocks};
use super::digest::digest16;
use super::error::BlkError;
use super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN};
use super::store_toc::{decode, TocEntry};
use super::wire::{MAX_READ_BYTES, SECTOR_SIZE};

pub struct StoreEntry {
    pub name: String,
    pub data: Vec<u8>,
}

// Hand-synced with the `--lba` flag mk/40-run.mk passes to nonos-store-pack.
// 256 keeps the container clear of blockfs's header ring, which rewrites
// LBA (generation % 256) on every commit and lands on 0 at generation 0.
const STORE_BASE_LBA: u64 = 256;

pub fn load() -> Result<Vec<StoreEntry>, BlkError> {
    let capacity_bytes = capacity()?
        .checked_mul(SECTOR_SIZE as u64)
        .ok_or(BlkError::BadLength)?;
    let mut head = [0u8; SECTOR_SIZE];
    read_blocks(STORE_BASE_LBA, &mut head)?;
    let count = entry_count(&head)?;
    let mut toc = vec![0u8; sector_span(HEADER_LEN + ENTRY_LEN * count)];
    read_blocks(STORE_BASE_LBA, &mut toc)?;
    let mut staged = Vec::with_capacity(count);
    for entry in decode(&toc, count, capacity_bytes)? {
        let data = read_extent(entry.offset, entry.len)?;
        verify(&entry, &data)?;
        staged.push(StoreEntry { name: entry.name, data });
    }
    Ok(staged)
}

fn verify(entry: &TocEntry, data: &[u8]) -> Result<(), BlkError> {
    if entry.digest == [0u8; 16] {
        return Ok(());
    }
    if digest16(data) == entry.digest {
        mark(b"[PKG] vfy ok ", &entry.name);
        Ok(())
    } else {
        mark(b"[PKG] vfy FAIL ", &entry.name);
        Err(BlkError::BadContainer)
    }
}

fn mark(tag: &[u8], name: &str) {
    let mut line = Vec::with_capacity(tag.len() + name.len() + 1);
    line.extend_from_slice(tag);
    line.extend_from_slice(name.as_bytes());
    line.push(b'\n');
    let _ = mk_debug(line.as_ptr(), line.len());
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
