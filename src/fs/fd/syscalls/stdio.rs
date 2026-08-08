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

extern crate alloc;

use alloc::format;

use crate::fs::fd::error::{FdError, FdResult};

pub(crate) fn write_stdout(buf: *const u8, count: usize) -> FdResult<usize> {
    if buf.is_null() {
        return Err(FdError::NullPointer);
    }

    // SAFETY: Caller guarantees buf is valid for count bytes
    unsafe {
        let slice = core::slice::from_raw_parts(buf, count);
        for &byte in slice {
            if byte == b'\n' {
                crate::sys::serial::print_str("\n");
            } else if byte.is_ascii_graphic() || byte == b' ' {
                let ch = byte as char;
                crate::sys::serial::print_str(&format!("{}", ch));
            }
        }
    }
    Ok(count)
}

pub(crate) fn write_stderr(buf: *const u8, count: usize) -> FdResult<usize> {
    if buf.is_null() {
        return Err(FdError::NullPointer);
    }

    // SAFETY: Caller guarantees buf is valid for count bytes
    unsafe {
        let slice = core::slice::from_raw_parts(buf, count);
        for &byte in slice {
            crate::arch::console::write_byte(byte);
        }
    }
    Ok(count)
}

pub(crate) fn read_stdin(buf: *mut u8, count: usize) -> FdResult<usize> {
    if buf.is_null() {
        return Err(FdError::NullPointer);
    }
    if count == 0 {
        return Ok(0);
    }

    // The PS/2 keyboard buffer is a legacy driver. The microkernel does
    // not own a keyboard input source on the trusted path; reading from
    // fd 0 returns "no data" (0 bytes), the canonical non-blocking
    // empty-stream answer.
    let _ = buf;
    Ok(0)
}
