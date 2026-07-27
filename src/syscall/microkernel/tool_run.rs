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

use super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT};
use crate::usercopy::{read_user_bytes, validate_user_read};

const MAX_NAME: usize = 48;
const MAX_ARGV: usize = 4096;

/// `MkToolRun(name_ptr, name_len, argv_ptr, argv_len)`. Run one of the baked,
/// attested command-line tools by its service name (for example `tool.tokei`),
/// parented to the caller so the caller feeds its stdin and drains its stdout
/// through the existing `MkProcInput`/`MkProcOutput` path. `argv` is the tool's
/// NUL-separated argument blob (`name\0arg1\0...`), empty for none. Returns the
/// new tool's pid, or an errno. Only the baked set can be named, so a caller can
/// never point this at an arbitrary binary.
pub fn sys_tool_run(name_ptr: u64, name_len: u64, argv_ptr: u64, argv_len: u64) -> i64 {
    let nlen = name_len as usize;
    if nlen == 0 || nlen > MAX_NAME || validate_user_read(name_ptr, nlen).is_err() {
        return ERRNO_INVAL;
    }
    let name = match read_user_bytes(name_ptr, nlen) {
        Ok(b) => b,
        Err(_) => return ERRNO_FAULT,
    };
    let alen = argv_len as usize;
    if alen > MAX_ARGV {
        return ERRNO_INVAL;
    }
    let argv = if alen == 0 {
        alloc::vec::Vec::new()
    } else {
        if validate_user_read(argv_ptr, alen).is_err() {
            return ERRNO_INVAL;
        }
        match read_user_bytes(argv_ptr, alen) {
            Ok(b) => b,
            Err(_) => return ERRNO_FAULT,
        }
    };
    match crate::userspace::tool_capsules::run_named(&name, &argv) {
        Some(pid) => pid as i64,
        None => ERRNO_NOENT,
    }
}
