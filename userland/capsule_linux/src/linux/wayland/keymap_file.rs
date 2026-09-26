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


//! Reading the keymap out of the store, once.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::{mk_debug, mk_getpid};

/// Where a keymap is installed. Produced by xkbcli compile-keymap and
/// packed into the store like any other data file.
const PATH: &[u8] = b"/usr/share/nonos/keymap.xkb";

/// An xkb keymap is tens of kilobytes; anything far larger is not one.
const MAX: u32 = 1 << 20;

pub fn load() -> Option<Vec<u8>> {
    match read_file(mk_getpid(), PATH, MAX) {
        Ok(bytes) if !bytes.is_empty() => Some(bytes),
        _ => {
            let line = b"[WAYLAND] no keymap in the store, keys will carry no symbols\n";
            let _ = mk_debug(line.as_ptr(), line.len());
            None
        }
    }
}
