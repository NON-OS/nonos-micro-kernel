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

#![no_std]
#![no_main]

extern crate alloc;

use nonos_libc::{heap_init, mk_debug, mk_exit, HeapError};
use nonos_toolkit::server;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    debug(b"[toolkit] start\n");
    match heap_init() {
        Ok(()) | Err(HeapError::AlreadyInitialized) => {}
        Err(_) => fail(1, b"[toolkit] heap fail\n"),
    }
    debug(b"[toolkit] server\n");
    server::runner::run();
}

fn debug(label: &[u8]) {
    let _ = mk_debug(label.as_ptr(), label.len());
}

fn fail(code: i32, label: &[u8]) -> ! {
    debug(label);
    mk_exit(code)
}
