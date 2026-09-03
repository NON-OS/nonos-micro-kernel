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
mod rule;

use core::mem::size_of;

use nonos_libc::{mk_proc_stat, ProcStatEntry, ProcStatHeader};

use rule::entry_names_installer;

// Matches the kernel's pid table bound; an entry is 64 bytes, so the stack
// buffer stays within a page and change.
const MAX_PROCS: usize = 64;
const HEADER_LEN: usize = size_of::<ProcStatHeader>();
const ENTRY_LEN: usize = size_of::<ProcStatEntry>();

// The sender pid is parsed by the kernel from the message envelope it stamps
// itself (`proc.<pid>`), so a capsule cannot forge it, and pid 0 is never
// handed to a process. Only the kernel-internal client enqueues without that
// prefix and so arrives as 0. Mutating operations answer it, or the attested
// installer, and nobody else: the package store at LBA 0 is read back as
// trusted input on the next boot.
pub fn permits(op: u16, sender_pid: u32) -> bool {
    if rule::allows(op, sender_pid, false) {
        return true;
    }
    rule::allows(op, sender_pid, sender_is_installer(sender_pid))
}

// The lookup runs on every mutating request. A cached verdict would outlive
// the installer's exit and follow its pid to whatever process the kernel
// hands that pid next; asking the kernel each time keeps the answer exactly
// as current as the process table.
fn sender_is_installer(sender_pid: u32) -> bool {
    let mut buf = [0u8; HEADER_LEN + MAX_PROCS * ENTRY_LEN];
    let written = mk_proc_stat(buf.as_mut_ptr(), MAX_PROCS as u32);
    if written <= 0 {
        return false;
    }
    let count = (written as usize).min(MAX_PROCS);
    for i in 0..count {
        let off = HEADER_LEN + i * ENTRY_LEN;
        // In bounds by construction of the buffer; read_unaligned because
        // the entry is packed into a byte stream, exactly as the kernel
        // wrote it.
        let e: ProcStatEntry =
            unsafe { core::ptr::read_unaligned(buf.as_ptr().add(off) as *const ProcStatEntry) };
        if e.pid == sender_pid {
            return entry_names_installer(&e.name, e.name_len);
        }
    }
    false
}
