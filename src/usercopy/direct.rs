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

//! Page-by-page byte transfer between kernel buffers and the
//! physical frames behind a user virtual address. Every transfer
//! window calls `walk::translate_read` or `walk::translate_write`
//! itself; "validate ran earlier" is not a substitute. Bytes move
//! at `DIRECTMAP_BASE + phys + offset`, never through the user VA.

use super::error::UsercopyError;
use super::walk::{translate_read, translate_write, UserLeaf};
use crate::memory::layout::DIRECTMAP_BASE;

pub(super) fn copy_from_user_directmap(user_ptr: u64, dst: &mut [u8]) -> Result<(), UsercopyError> {
    transfer(user_ptr, dst.len(), translate_read, |leaf, off, n| {
        let src = (DIRECTMAP_BASE + leaf.phys_base + leaf.offset) as *const u8;
        // SAFETY: ek@nonos.systems — `leaf` came from `translate_read`
        // so the underlying page is mapped, USER, and reachable
        // through the directmap. `n` does not exceed the bytes
        // remaining in the leaf page.
        unsafe { core::ptr::copy_nonoverlapping(src, dst[off..].as_mut_ptr(), n) };
    })
}

pub(super) fn copy_to_user_directmap(user_ptr: u64, src: &[u8]) -> Result<(), UsercopyError> {
    transfer(user_ptr, src.len(), translate_write, |leaf, off, n| {
        let dst = (DIRECTMAP_BASE + leaf.phys_base + leaf.offset) as *mut u8;
        // SAFETY: ek@nonos.systems — `leaf` came from `translate_write`
        // so the page is mapped, USER, WRITABLE.
        unsafe { core::ptr::copy_nonoverlapping(src[off..].as_ptr(), dst, n) };
    })
}

fn transfer<T, S>(user_ptr: u64, len: usize, translate: T, mut step: S) -> Result<(), UsercopyError>
where
    T: Fn(u64) -> Result<UserLeaf, UsercopyError>,
    S: FnMut(&UserLeaf, usize, usize),
{
    let mut cursor = 0usize;
    while cursor < len {
        let va = user_ptr.checked_add(cursor as u64).ok_or(UsercopyError::AddressOverflow)?;
        let leaf = translate(va)?;
        let remaining = leaf.bytes_remaining_in_page() as usize;
        let n = remaining.min(len - cursor);
        step(&leaf, cursor, n);
        cursor += n;
    }
    Ok(())
}
