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

//! Which program to run.

use alloc::vec::Vec;

use nonos_libc::mk_args;

use crate::linux::file::{key, store_read, visible};

use super::origin::Origin;

/// The built-in program: Alpine's static busybox, embedded so a machine with
/// nothing in the store still runs a real Linux binary.
static BUILT_IN: &[u8] = include_bytes!("../../guests/busybox.elf");

const MAX_IMAGE: u32 = 64 << 20;
const MAX_ARGS: usize = 256;

/// The program's path, its bytes, and where they came from.
pub fn source() -> (Vec<u8>, Vec<u8>, Origin) {
    match named() {
        Some((path, bytes)) => (path, bytes, Origin::Store),
        None => (b"/bin/busybox".to_vec(), BUILT_IN.to_vec(), Origin::BuiltIn),
    }
}

fn named() -> Option<(Vec<u8>, Vec<u8>)> {
    let mut buf = [0u8; MAX_ARGS];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return None;
    }
    // The first argument is the path.
    let args = &buf[..n as usize];
    let end = args.iter().position(|b| *b == 0 || *b == b' ').unwrap_or(args.len());
    let path = args.get(..end)?;
    if path.is_empty() {
        return None;
    }
    /*
     * The argument is not a guest's, but the program it names is a
     * Linux one and lives where Linux programs live.
     */
    let at = visible(b"/", path);
    let bytes = store_read(&key(&at), MAX_IMAGE).ok()?;
    Some((at, bytes))
}
