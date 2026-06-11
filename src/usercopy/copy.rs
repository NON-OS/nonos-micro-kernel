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

//! Byte-slice copy between kernel buffers and user buffers. The user
//! virtual address is never dereferenced; transfers go through the
//! directmap helpers in `direct.rs` after `validate.rs` has cleared
//! the range and access policy.

use super::direct::{copy_from_user_directmap, copy_to_user_directmap};
use super::error::UsercopyError;
use super::validate::{validate_user_read, validate_user_write};
use crate::arch::run_without_interrupts;

pub fn copy_from_user(user_ptr: u64, dst: &mut [u8]) -> Result<(), UsercopyError> {
    run_without_interrupts(|| {
        validate_user_read(user_ptr, dst.len())?;
        copy_from_user_directmap(user_ptr, dst)
    })
}

pub fn copy_to_user(user_ptr: u64, src: &[u8]) -> Result<(), UsercopyError> {
    run_without_interrupts(|| {
        validate_user_write(user_ptr, src.len())?;
        copy_to_user_directmap(user_ptr, src)
    })
}
