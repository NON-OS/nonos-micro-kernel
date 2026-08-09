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

//! Appends a named payload to the on-disk NONOSTR1 store, committing sector 0 last.
use alloc::vec;

use nonos_libc::mk_store_write;

use super::client::{capacity, read_blocks};
use super::digest::digest16;
use super::error::BlkError;
use super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN, MAX_ENTRIES};
use super::store_toc::{decode, valid_name, TocEntry, MAX_TOTAL_BYTES, NAME_LEN};
use super::wire::SECTOR_SIZE;

const STORE_BASE_LBA: u64 = 256;
const MAX_WRITE_BYTES: usize = 8192;

pub fn append(name: &str, data: &[u8]) -> Result<(), BlkError> {
    let mut head = [0u8; SECTOR_SIZE];
    read_blocks(STORE_BASE_LBA, &mut head)?;
    let count = entry_count(&head)?;
    let mut toc = vec![0u8; sector_span(HEADER_LEN + ENTRY_LEN * count)];
    read_blocks(STORE_BASE_LBA, &mut toc)?;
    let capacity_bytes = capacity()?
        .checked_mul(SECTOR_SIZE as u64)
        .ok_or(BlkError::BadLength)?;
    let region_len = sector_span(HEADER_LEN + ENTRY_LEN * (count + 1));
    let reserved = sector_span(HEADER_LEN + ENTRY_LEN * MAX_ENTRIES);
    let mut next_off = STORE_BASE_LBA * SECTOR_SIZE as u64 + reserved as u64;
    let mut committed = 0u64;
    for entry in decode(&toc, count, capacity_bytes)? {
        if entry.name == name {
            return same_bytes(&entry, data);
        }
        committed += entry.len;
        next_off = next_off.max(align_up(entry.offset + entry.len, SECTOR_SIZE));
    }
    if committed.saturating_add(data.len() as u64) > MAX_TOTAL_BYTES {
        return Err(BlkError::BadLength);
    }
    if count >= MAX_ENTRIES || !valid_name(name) {
        return Err(BlkError::BadContainer);
    }
    write_payload(next_off, data)?;
    let mut region = vec![0u8; region_len];
    region[..toc.len()].copy_from_slice(&toc);
    let base = HEADER_LEN + ENTRY_LEN * count;
    region[base..base + name.len()].copy_from_slice(name.as_bytes());
    region[base + NAME_LEN..base + NAME_LEN + 8].copy_from_slice(&next_off.to_le_bytes());
    region[base + NAME_LEN + 8..base + NAME_LEN + 16]
        .copy_from_slice(&(data.len() as u64).to_le_bytes());
    region[base + NAME_LEN + 16..base + NAME_LEN + 32].copy_from_slice(&digest16(data));
    region[12..16].copy_from_slice(&(count as u32 + 1).to_le_bytes());
    commit(&region)
}

// A TOC name is written once. Re-persisting the exact bytes already committed
// is the idempotent retry the store_persist path relies on, so it succeeds;
// anything else would have to rewrite an extent in place, which this appender
// cannot do, and reporting success there would leave the caller believing the
// old bytes on disk had been replaced.
fn same_bytes(entry: &TocEntry, data: &[u8]) -> Result<(), BlkError> {
    if entry.len == data.len() as u64 && entry.digest == digest16(data) {
        return Ok(());
    }
    Err(BlkError::Exists)
}

fn write_payload(next_off: u64, data: &[u8]) -> Result<(), BlkError> {
    let mut lba = next_off / SECTOR_SIZE as u64;
    let mut done = 0usize;
    while done < data.len() {
        let take = core::cmp::min(MAX_WRITE_BYTES, data.len() - done);
        let mut buf = vec![0u8; sector_span(take)];
        buf[..take].copy_from_slice(&data[done..done + take]);
        write_sectors(lba, &buf)?;
        lba += (buf.len() / SECTOR_SIZE) as u64;
        done += take;
    }
    Ok(())
}

pub(super) fn commit(region: &[u8]) -> Result<(), BlkError> {
    let mut off = SECTOR_SIZE;
    while off < region.len() {
        let take = core::cmp::min(MAX_WRITE_BYTES, region.len() - off);
        let lba = STORE_BASE_LBA + (off / SECTOR_SIZE) as u64;
        write_sectors(lba, &region[off..off + take])?;
        off += take;
    }
    write_sectors(STORE_BASE_LBA, &region[..SECTOR_SIZE])
}

fn write_sectors(lba: u64, buf: &[u8]) -> Result<(), BlkError> {
    let rc = mk_store_write(lba, buf.as_ptr(), buf.len());
    if rc < 0 {
        return Err(BlkError::Transport(rc));
    }
    Ok(())
}

fn sector_span(bytes: usize) -> usize {
    bytes.div_ceil(SECTOR_SIZE) * SECTOR_SIZE
}

fn align_up(v: u64, a: usize) -> u64 {
    let a = a as u64;
    v.div_ceil(a) * a
}
