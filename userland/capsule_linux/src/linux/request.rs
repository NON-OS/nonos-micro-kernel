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


//! Reading what this capsule was asked to do.

use alloc::string::String;

use nonos_libc::mk_args;

/// Matches the buffer the program path is read into.
const MAX_ARGS: usize = 256;

/// Arguments `install` then `<name>` ask this capsule to fetch a package
/// rather than run a program.
pub fn install_request() -> Option<String> {
    let mut buf = [0u8; MAX_ARGS];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return None;
    }
    let mut parts = buf[..n as usize].split(|b| *b == 0);
    if parts.next()? != b"install" {
        return None;
    }
    let name = parts.next().filter(|s| !s.is_empty())?;
    Some(String::from(core::str::from_utf8(name).ok()?))
}
