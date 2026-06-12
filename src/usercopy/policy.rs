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

//! Pure range policy for user buffers. No page walking. No
//! permission decision. Used by `validate.rs` and `string.rs` so
//! both error in the same way for null pointers, zero / oversized
//! lengths, address overflow, and ranges that cross the canonical
//! user limit. Permission and presence checks live in `walk`.

use super::error::UsercopyError;

pub(super) const USER_SPACE_END: u64 = 0x0000_7FFF_FFFF_FFFF;
pub(super) const PAGE_SIZE: u64 = 4096;
pub(super) const MAX_COPY_SIZE: usize = 64 * 1024 * 1024;

#[derive(Clone, Copy)]
pub(super) struct UserRange {
    pub start_page: u64,
    pub end_page: u64,
}

pub(super) fn check_range(addr: u64, len: usize) -> Result<Option<UserRange>, UsercopyError> {
    if addr == 0 {
        return Err(UsercopyError::NullPointer);
    }
    if len > MAX_COPY_SIZE {
        return Err(UsercopyError::SizeTooLarge);
    }
    if len == 0 {
        return Ok(None);
    }
    let end = addr.checked_add(len as u64 - 1).ok_or(UsercopyError::AddressOverflow)?;
    if end > USER_SPACE_END {
        return Err(UsercopyError::InvalidAddress);
    }
    Ok(Some(UserRange { start_page: addr & !0xFFF, end_page: end & !0xFFF }))
}
