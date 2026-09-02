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

use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();

/// The terminal rail's cap, so both readers ask the kernel for the same table.
const MAX_PROCS: usize = 64;

pub struct Runtime {
    pub capsules: u32,
    pub mem_total_kb: u64,
    pub mem_used_kb: u64,
    pub load: [u64; 3],
}

/// One read of the live process table. Sizes are kibibytes exactly as the kernel
/// publishes them, and `load` stays raw Q11 (2048 == 1.00) so the formatter owns
/// the rounding. `mem_used_kb` is the sum of the per-entry resident sizes, hence
/// a lower bound whenever more than `MAX_PROCS` processes are live.
pub fn sample() -> Option<Runtime> {
    let mut buf = [0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
    let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
    if written <= 0 {
        return None;
    }
    let header: ProcStatHeader =
        unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const ProcStatHeader) };
    let count = (written as usize).min(MAX_PROCS);
    let mut mem_used_kb = 0u64;
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        if off + ENTRY_LEN > buf.len() {
            break;
        }
        let entry: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        mem_used_kb = mem_used_kb.saturating_add(entry.mem_kb);
    }
    Some(Runtime {
        capsules: header.count,
        mem_total_kb: header.mem_total_kb,
        mem_used_kb,
        load: header.load_avg_fixed,
    })
}
