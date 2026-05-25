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

mod alloc;
mod install;
mod validate;

use super::records;
use super::types::{DmaGrant, DmaMapError, DmaMapRequest, DmaMapResult};

// `MkDmaMap`: validate -> alloc+zero frames -> install user pages ->
// record. Each step is a single responsibility in its own file; this
// function is the transaction boundary and owns the rollback chain.
pub fn map_for_caller(pid: u32, req: DmaMapRequest) -> Result<DmaMapResult, DmaMapError> {
    let claim_epoch = match validate::validate(&req, pid) {
        Ok(epoch) => epoch,
        Err(e) => return fail("validate", e),
    };
    let pages = req.length / validate::PAGE_SIZE;

    let phys_start = match alloc::alloc_and_zero(pages, req.length, req.flags) {
        Ok(start) => start,
        Err(e) => return fail("alloc", e),
    };

    let user_va = match install::install(pages, req.length, phys_start) {
        Ok(va) => va,
        Err(e) => {
            alloc::free(phys_start, pages);
            return fail("install", e);
        }
    };

    let grant_id = records::allocate_id();
    records::insert(DmaGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch,
        physical_start: phys_start,
        user_va,
        length: req.length,
        flags: req.flags,
    });

    Ok(DmaMapResult { user_va, device_addr: phys_start, length: req.length, grant_id })
}

fn fail(stage: &str, error: DmaMapError) -> Result<DmaMapResult, DmaMapError> {
    if stage == "alloc" && error == DmaMapError::NoMemory {
        let (start, end) = crate::memory::phys::managed_range();
        crate::sys::serial::print(b"[DMA] free-frames=");
        crate::sys::serial::print_dec(crate::memory::phys::total_free_frames() as u64);
        crate::sys::serial::print(b" max-run=");
        crate::sys::serial::print_dec(crate::memory::phys::largest_free_run() as u64);
        crate::sys::serial::print(b" range=");
        crate::sys::serial::print_hex(start);
        crate::sys::serial::print(b"..");
        crate::sys::serial::print_hex(end);
        crate::sys::serial::println(b"");
    }
    crate::sys::serial::println(match (stage, error) {
        ("validate", DmaMapError::BadLengthForClass) => b"[DMA] validate bad-length-class",
        ("validate", DmaMapError::BadLength) => b"[DMA] validate bad-length",
        ("validate", DmaMapError::NotClaimed) => b"[DMA] validate not-claimed",
        ("validate", DmaMapError::StaleEpoch) => b"[DMA] validate stale-epoch",
        ("validate", DmaMapError::UnknownDevice) => b"[DMA] validate unknown-device",
        ("alloc", DmaMapError::NoMemory) => b"[DMA] alloc no-memory",
        ("install", DmaMapError::NoVaSpace) => b"[DMA] install no-va-space",
        ("install", DmaMapError::MapFailed) => b"[DMA] install map-failed",
        _ => b"[DMA] map failed",
    });
    Err(error)
}
