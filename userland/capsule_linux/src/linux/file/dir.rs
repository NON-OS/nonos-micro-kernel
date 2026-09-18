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


//! Opening a directory.
//!
//! The listing is taken once, when the directory is opened, and the guest
//! walks that copy. Linux promises no more: a directory stream shows what
//! was there when it was opened, and what arrives afterwards may or may
//! not appear.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::list_paths;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::slot;

pub fn open(guest: &mut Guest, owner: u32, path: Vec<u8>) -> u64 {
    let Ok(keys) = list_paths(owner, &path) else {
        return errno::fail(errno::EACCES);
    };
    let names = children(&path, keys);
    match slot::install(guest, Fd::dir(path, names)) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

/*
 * The store answers with whole keys under the prefix, at any depth. A
 * directory holds its immediate children, so a key that still has a
 * separator after the prefix belongs to a subdirectory and is reported
 * as that subdirectory, once.
 */
fn children(path: &[u8], keys: Vec<String>) -> Vec<String> {
    let cut = if path == b"/" { 1 } else { path.len() + 1 };
    let mut out: Vec<String> = Vec::new();
    for key in keys {
        let bytes = key.as_bytes();
        if bytes.len() <= cut {
            continue;
        }
        let rest = &bytes[cut..];
        let end = rest.iter().position(|b| *b == b'/').unwrap_or(rest.len());
        let Ok(name) = core::str::from_utf8(&rest[..end]) else {
            continue;
        };
        if !out.iter().any(|seen| seen == name) {
            out.push(String::from(name));
        }
    }
    out
}
