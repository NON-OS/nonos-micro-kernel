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

use alloc::vec::Vec;

use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

const MAX_ARTIFACT: usize = 16 * 1024 * 1024;

// Copy one artifact blob out of user memory after bounds-checking the length and
// validating the user range. Returns a negative errno on a bad length or fault.
pub(crate) fn read_blob(ptr: u64, len: u32) -> Result<Vec<u8>, i64> {
    let n = len as usize;
    if n == 0 || n > MAX_ARTIFACT {
        return Err(ERRNO_INVAL);
    }
    if crate::usercopy::validate_user_read(ptr, n).is_err() {
        return Err(ERRNO_FAULT);
    }
    crate::usercopy::read_user_bytes(ptr, n).map_err(|_| ERRNO_FAULT)
}
