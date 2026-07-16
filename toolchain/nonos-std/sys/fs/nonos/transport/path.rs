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

// Path helpers for the VFS backend: read the process working directory
// (MGCW), turn a std Path into the service's length-checked byte form
// (joining a relative path onto the cwd so chdir takes effect), and build
// the pid+path request body most operations start from.

use super::err::err;
use super::syscall::{sys3, tag4};
use crate::io;
use crate::path::Path;
use crate::vec::Vec;

const N_MK_GETCWD: i64 = tag4(b"MGCW");

// This process's working directory, for resolving a relative path. Empty if
// the kernel has none, in which case a relative path is taken from the root.
pub(crate) fn cwd_bytes() -> Vec<u8> {
    let mut buf = [0u8; 255];
    let n = unsafe { sys3(N_MK_GETCWD, buf.as_mut_ptr() as u64, buf.len() as u64, 0) };
    if n <= 0 {
        return Vec::new();
    }
    buf[..(n as usize).min(buf.len())].to_vec()
}

pub(crate) fn path_bytes(p: &Path) -> io::Result<Vec<u8>> {
    let b = p.as_os_str().as_encoded_bytes();
    if b.is_empty() || b.len() > 255 {
        return Err(err("bad path"));
    }
    // Absolute paths go through untouched; a relative path is joined onto the
    // process working directory so `chdir` actually changes what it opens.
    if b[0] == b'/' {
        return Ok(b.to_vec());
    }
    let mut out = cwd_bytes();
    if out.is_empty() || out.last() != Some(&b'/') {
        out.push(b'/');
    }
    out.extend_from_slice(b);
    if out.len() > 255 {
        return Err(err("bad path"));
    }
    Ok(out)
}

pub(crate) fn path_body(pid: u32, p: &Path) -> io::Result<Vec<u8>> {
    let path = path_bytes(p)?;
    let mut body = Vec::with_capacity(5 + path.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(&path);
    Ok(body)
}
