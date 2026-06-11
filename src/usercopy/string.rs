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

//! Read a NUL-terminated user string. Range rules come from
//! `policy::check_range`; per-page translation comes from
//! `walk::translate_read`. The kernel never dereferences the user
//! virtual address directly; the byte scan reads at
//! `DIRECTMAP_BASE + phys + offset`, bounded by the bytes remaining
//! in the current leaf.

use super::error::UsercopyError;
use super::policy::check_range;
use super::walk::translate_read;
use crate::arch::run_without_interrupts;
use crate::memory::layout::DIRECTMAP_BASE;

const MAX_STRING_LEN: usize = 4096;

pub fn read_user_string(
    user_ptr: u64,
    max_len: usize,
) -> Result<alloc::string::String, UsercopyError> {
    let safe_len = max_len.min(MAX_STRING_LEN);
    if check_range(user_ptr, safe_len)?.is_none() {
        return Ok(alloc::string::String::new());
    }
    let mut buf = alloc::vec![0u8; safe_len];
    let actual_len = run_without_interrupts(|| scan_until_nul(user_ptr, safe_len, &mut buf))?;
    buf.truncate(actual_len);
    alloc::string::String::from_utf8(buf).map_err(|_| UsercopyError::InvalidUtf8)
}

fn scan_until_nul(user_ptr: u64, safe_len: usize, buf: &mut [u8]) -> Result<usize, UsercopyError> {
    let mut cursor = 0usize;
    while cursor < safe_len {
        let va = user_ptr.checked_add(cursor as u64).ok_or(UsercopyError::AddressOverflow)?;
        let leaf = translate_read(va)?;
        let bytes_in_page = leaf.bytes_remaining_in_page() as usize;
        let take = bytes_in_page.min(safe_len - cursor);
        let src = (DIRECTMAP_BASE + leaf.phys_base + leaf.offset) as *const u8;
        for i in 0..take {
            // SAFETY: ek@nonos.systems — `leaf` came from
            // `translate_read`; the read targets directmap memory
            // covered by the caller's CR3. The byte read is volatile
            // so the compiler does not collapse the NUL scan.
            let byte = unsafe { core::ptr::read_volatile(src.add(i)) };
            if byte == 0 {
                return Ok(cursor + i);
            }
            buf[cursor + i] = byte;
        }
        cursor += take;
    }
    Ok(cursor)
}
