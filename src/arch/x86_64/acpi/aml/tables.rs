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

//! Locating and safely copying the AML bytecode blocks out of firmware tables.

use alloc::vec::Vec;
use core::ptr;

use crate::arch::x86_64::acpi::parser::phys::directmap;
use crate::arch::x86_64::acpi::parser::state::TABLES;
use crate::arch::x86_64::acpi::tables::{Fadt, SdtHeader, SIG_FADT};

/// Size of the standard SDT header that precedes every AML block.
const SDT_HEADER_LEN: usize = 36;

/// Reject any AML block claiming to be larger than this. Real DSDT/SSDT tables
/// on laptops are tens to low hundreds of kilobytes; anything past 2 MiB is a
/// corrupt length field and must not be trusted for a copy.
const MAX_AML_LEN: usize = 2 * 1024 * 1024;

/// Return the DSDT AML bytecode as an owned copy.
///
/// The DSDT is not present in the XSDT-derived table registry; its physical
/// address lives in the FADT. We fetch the FADT physical address from the
/// registry, re-read the FADT to obtain `dsdt_address()`, then copy the AML
/// payload that follows the DSDT's own SDT header.
///
/// Returns `None` on any failure (missing FADT, unmapped physical address,
/// implausible length). Never panics.
pub fn dsdt_aml() -> Option<Vec<u8>> {
    let fadt_phys = { TABLES.read().as_ref()?.tables.get(&SIG_FADT).copied()? };
    let fadt_virt = directmap(fadt_phys)?;

    // SAFETY: `fadt_virt` is a direct-map virtual address for a firmware FADT
    // whose physical address was recorded by the validated ACPI parser. The
    // FADT is at least the fixed struct size for any conforming firmware; we
    // read it by value with a volatile read to avoid UB from aliasing MMIO.
    let dsdt_phys = unsafe { ptr::read_volatile(fadt_virt as *const Fadt).dsdt_address() };
    if dsdt_phys == 0 {
        return None;
    }

    copy_aml_block(dsdt_phys)
}

/// Copy the AML payload of an SDT located at `phys`.
///
/// Reads the 36-byte SDT header, validates the declared length, and copies the
/// `length - 36` bytes of AML that follow. Returns `None` if the physical
/// address does not map, if the length is smaller than a header, or if the
/// length exceeds `MAX_AML_LEN`.
fn copy_aml_block(phys: u64) -> Option<Vec<u8>> {
    let virt = directmap(phys)?;

    // SAFETY: `virt` is a direct-map address for a firmware ACPI table. We read
    // the fixed 36-byte header by value; the parser previously validated that a
    // table exists at this physical address.
    let header = unsafe { ptr::read_volatile(virt as *const SdtHeader) };
    let total = header.length as usize;
    if total < SDT_HEADER_LEN || total > MAX_AML_LEN {
        return None;
    }

    let aml_len = total - SDT_HEADER_LEN;
    if aml_len == 0 {
        return None;
    }

    let aml_start = virt.checked_add(SDT_HEADER_LEN as u64)?;

    let mut buf = Vec::new();
    buf.try_reserve_exact(aml_len).ok()?;

    // SAFETY: The header's length field was bounds-checked above, so the mapped
    // table spans at least `SDT_HEADER_LEN + aml_len` bytes starting at `virt`.
    // The source and destination do not overlap; `buf` was reserved for exactly
    // `aml_len` bytes and its length is set only after the copy completes.
    unsafe {
        ptr::copy_nonoverlapping(aml_start as *const u8, buf.as_mut_ptr(), aml_len);
        buf.set_len(aml_len);
    }

    Some(buf)
}

/// Collect the AML blocks that this bounded extractor is able to reach.
///
/// SIMPLIFICATION: only the DSDT is returned. SSDTs are referenced by the XSDT
/// but the table registry keys tables by signature, so colliding SSDT
/// signatures overwrite one another and the individual SSDT physical addresses
/// are not recoverable from that map. Full SSDT enumeration would require
/// walking the raw XSDT entry array (see parser/root_xsdt.rs). The touchpad is
/// declared in the DSDT on the vast majority of laptops, so this is sufficient
/// for the touchpad-enumeration goal while leaving the gap documented.
pub fn aml_blocks() -> Vec<Vec<u8>> {
    let mut blocks = Vec::new();
    if let Some(dsdt) = dsdt_aml() {
        blocks.push(dsdt);
    }
    // Every SSDT the root table pointed at. The parser records their physical
    // addresses in full (the signature-keyed map cannot, since SSDTs collide),
    // so a touchpad described in a secondary SSDT is reachable too. Copy the
    // addresses out under the lock, then map each without holding it.
    let ssdt_phys = match TABLES.read().as_ref() {
        Some(reg) => reg.ssdt_addresses.clone(),
        None => Vec::new(),
    };
    for phys in ssdt_phys {
        if let Some(ssdt) = copy_aml_block(phys) {
            blocks.push(ssdt);
        }
    }
    blocks
}
