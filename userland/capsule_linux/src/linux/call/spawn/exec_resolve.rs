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

//! Which image actually runs, once `#!` has had its say.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::file::{key, store_read, visible};

use super::exec_shebang::parse;

/// The same ceiling the first image is read under.
pub const MAX_IMAGE: u32 = 64 << 20;

/// Linux's own limit on how deep `#!` may point at another script.
const MAX_DEPTH: usize = 4;

pub struct Program {
    pub path: Vec<u8>,
    pub bytes: Vec<u8>,
    pub argv: Vec<Vec<u8>>,
}

pub fn resolve(cwd: &[u8], name: &[u8], argv: &[Vec<u8>]) -> Result<Program, u64> {
    let mut path = visible(cwd, name);
    let mut args = argv.to_vec();
    for _ in 0..MAX_DEPTH {
        let Ok(bytes) = store_read(&key(&path), MAX_IMAGE) else {
            return Err(errno::fail(errno::ENOENT));
        };
        if crate::linux::attest::verify(&path, &bytes).is_err() {
            return Err(errno::fail(errno::EPERM));
        }
        let Some(interp) = parse(&bytes) else {
            return Ok(Program { path, bytes, argv: args });
        };
        args = rewrite(&interp, &path, &args);
        path = visible(cwd, &interp.path);
    }
    Err(errno::fail(errno::ELOOP))
}

/// The interpreter, its one optional argument, the script, then whatever the
/// caller passed after argv[0].
fn rewrite(interp: &super::exec_shebang::Interp, script: &[u8], args: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut out = alloc::vec![interp.path.clone()];
    out.extend(interp.arg.clone());
    out.push(script.to_vec());
    out.extend(args.iter().skip(1).cloned());
    out
}
