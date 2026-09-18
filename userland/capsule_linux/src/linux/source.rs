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
//!
//! The path comes from this capsule's own arguments, so the personality
//! runs whatever it was asked to run and is not a wrapper around one
//! binary. With no argument it falls back to the image built into it,
//! which exists so the mechanism can be proved on a machine with nothing
//! in the store yet.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::{mk_args, mk_getpid};

/// The proof image: a static Linux executable, embedded so the first run
/// needs no disk.
static BUILT_IN: &[u8] = include_bytes!("../../guests/hello.elf");

const MAX_IMAGE: u32 = 64 << 20;
const MAX_ARGS: usize = 256;

/// The program's path and its bytes.
pub fn source() -> (Vec<u8>, Vec<u8>) {
    match named() {
        Some(pair) => pair,
        None => (b"/guest/hello".to_vec(), BUILT_IN.to_vec()),
    }
}

fn named() -> Option<(Vec<u8>, Vec<u8>)> {
    let mut buf = [0u8; MAX_ARGS];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return None;
    }
    /*
     * The first argument is the path. Anything after it is the guest's
     * own business and is not passed on yet: a program's argument vector
     * has to be built into its stack before it starts, and only the path
     * is needed to get it there.
     */
    let args = &buf[..n as usize];
    let end = args.iter().position(|b| *b == 0 || *b == b' ').unwrap_or(args.len());
    let path = args.get(..end)?;
    if path.is_empty() {
        return None;
    }
    let bytes = read_file(mk_getpid() as u32, path, MAX_IMAGE).ok()?;
    Some((path.to_vec(), bytes))
}
