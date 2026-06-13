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

use alloc::string::String;
use alloc::vec;
use alloc::vec::{IntoIter, Vec};

pub struct Args {
    inner: IntoIter<String>,
}

impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.inner.next()
    }
}

pub fn args() -> Args {
    Args { inner: collect().into_iter() }
}

fn collect() -> Vec<String> {
    let needed = nonos_libc::mk_args(core::ptr::null_mut(), 0);
    if needed <= 0 {
        return Vec::new();
    }
    let mut buf = vec![0u8; needed as usize];
    let n = nonos_libc::mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return Vec::new();
    }
    buf.truncate(n as usize);
    buf.split(|&b| b == 0).map(|s| String::from_utf8_lossy(s).into_owned()).collect()
}
