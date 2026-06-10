// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::table::boot::{AllocateType, BootServices, MemoryType};

/// Allocate and copy kernel command line. Returns 0 on failure or if no cmdline.
pub fn allocate_cmdline(bs: &BootServices, cmdline: Option<&str>) -> u64 {
    let s = match cmdline {
        Some(s) => s,
        None => return 0,
    };
    let cmd_bytes = s.as_bytes();
    let cmd_len = cmd_bytes.len() + 1;
    let cmd_pages = (cmd_len + 0xFFF) / 0x1000;
    match bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, cmd_pages) {
        Ok(cmd_addr) => {
            // SAFETY: cmd_addr from allocate_pages is valid for cmd_pages * 4096 bytes
            unsafe {
                let ptr = cmd_addr as *mut u8;
                core::ptr::copy_nonoverlapping(cmd_bytes.as_ptr(), ptr, cmd_bytes.len());
                core::ptr::write_volatile(ptr.add(cmd_bytes.len()), 0u8);
            }
            cmd_addr
        }
        Err(_) => 0,
    }
}
