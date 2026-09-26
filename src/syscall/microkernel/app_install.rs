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

//! `MkAppInstall`: ask for a distribution package to be installed.

use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_FAULT, ERRNO_INVAL};
use crate::usercopy::{read_user_bytes, validate_user_read};

/// Long enough for any real package name and short enough that the
/// argument cannot become a payload.
const MAX_NAME: usize = 64;

/// `MkAppInstall(name_ptr, name_len)`.
pub fn sys_app_install(name_ptr: u64, name_len: u64) -> i64 {
    let len = name_len as usize;
    if len == 0 || len > MAX_NAME || validate_user_read(name_ptr, len).is_err() {
        return ERRNO_INVAL;
    }
    let Ok(raw) = read_user_bytes(name_ptr, len) else {
        return ERRNO_FAULT;
    };
    /*
     * The name reaches a URL and a store path, so it is held to what a package
     * name actually is.
     */
    if !raw.iter().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'+' | b'.')) {
        return ERRNO_INVAL;
    }
    if raw.first() == Some(&b'.') {
        return ERRNO_INVAL;
    }
    let Ok(name) = alloc::string::String::from_utf8(raw) else {
        return ERRNO_INVAL;
    };
    match crate::userspace::init::request_install(name) {
        true => 0,
        false => ERRNO_BUSY,
    }
}
