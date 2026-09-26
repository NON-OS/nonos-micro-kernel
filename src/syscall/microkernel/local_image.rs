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

//! The image argument both local-attestation calls take.

use alloc::vec::Vec;

use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

/// A Linux program with its interpreter is comfortably inside this.
pub(super) const MAX_ELF: usize = 64 << 20;

pub(super) fn copy_in(ptr: u64, len: u64, cap: usize) -> Result<Vec<u8>, i64> {
    if len == 0 || len as usize > cap {
        return Err(ERRNO_INVAL);
    }
    let mut out = alloc::vec![0u8; len as usize];
    match crate::usercopy::copy_from_user(ptr, &mut out) {
        Ok(()) => Ok(out),
        Err(_) => Err(ERRNO_FAULT),
    }
}

/// A caller may not mint or claim authority it does not itself hold.
pub(super) fn within_own_authority(caps: u64) -> bool {
    let held = crate::capabilities::caps_to_bits(
        &crate::syscall::caps::current_caps_or_default().permissions,
    );
    caps & !held == 0
}
