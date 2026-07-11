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

use core::sync::atomic::Ordering;

use super::super::super::spec::SpawnError;
use crate::elf::loader::load_elf_entry_into;
use crate::memory::paging::manager::lookup_asid_for_process;

pub(super) fn load_elf_into_pid(
    elf: &'static [u8],
    pid: u32,
    debug_tag: &'static [u8],
) -> Result<u64, SpawnError> {
    let asid = lookup_asid_for_process(pid).ok_or(SpawnError::AddressSpace)?;
    // load_elf_entry_into maps the segments AND applies relative relocations and
    // RELRO; the loader must stay this one, since skipping either leaves null GOT
    // entries that fault the capsule at cr2=0.
    let entry = load_elf_entry_into(elf, asid).map_err(|err| {
        crate::sys::serial::println(debug_tag);
        crate::sys::serial::println(err.as_str().as_bytes());
        SpawnError::ElfLoad
    })?;
    // Record the committed image footprint as this process's resident memory, so
    // the process monitor reports the code, data and bss a capsule maps at load
    // instead of zero. This is a read-only scan of the same PT_LOAD headers the
    // loader just mapped; later mmap/munmap adjust the same counter.
    let pages = image_load_pages(elf);
    if pages != 0 {
        crate::process::with_process(pid, |pcb| {
            pcb.memory.lock().resident_pages.store(pages, Ordering::Relaxed);
        });
    }
    Ok(entry)
}

// Total virtual span, in 4 KiB pages, covered by the ELF's PT_LOAD segments.
// Pure bounds-checked reads of the ELF64 header and program table; a malformed
// or truncated field yields 0 and leaves the resident count untouched.
fn image_load_pages(elf: &[u8]) -> u64 {
    const PT_LOAD: u32 = 1;
    let rd16 =
        |o: usize| -> Option<u16> { elf.get(o..o + 2).map(|s| u16::from_le_bytes([s[0], s[1]])) };
    let rd32 = |o: usize| -> Option<u32> {
        elf.get(o..o + 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    };
    let rd64 = |o: usize| -> Option<u64> {
        elf.get(o..o + 8).map(|s| {
            let mut a = [0u8; 8];
            a.copy_from_slice(s);
            u64::from_le_bytes(a)
        })
    };

    let (phoff, phentsize, phnum) = match (rd64(0x20), rd16(0x36), rd16(0x38)) {
        (Some(off), Some(ent), Some(num)) => (off as usize, ent as usize, num as usize),
        _ => return 0,
    };
    if phentsize < 0x38 {
        return 0;
    }

    let mut lo = u64::MAX;
    let mut hi = 0u64;
    for i in 0..phnum {
        let base = match phoff.checked_add(i * phentsize) {
            Some(b) => b,
            None => break,
        };
        let p_type = match rd32(base) {
            Some(v) => v,
            None => break,
        };
        if p_type != PT_LOAD {
            continue;
        }
        let vaddr = match rd64(base + 0x10) {
            Some(v) => v,
            None => break,
        };
        let memsz = match rd64(base + 0x28) {
            Some(v) => v,
            None => break,
        };
        if memsz == 0 {
            continue;
        }
        lo = lo.min(vaddr);
        hi = hi.max(vaddr.saturating_add(memsz));
    }
    if hi <= lo {
        0
    } else {
        (hi - lo).div_ceil(4096)
    }
}
