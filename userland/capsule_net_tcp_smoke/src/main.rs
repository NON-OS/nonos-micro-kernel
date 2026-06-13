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

use nonos_libc::{heap_init, mk_debug, mk_exit};

const BOOT: &[u8] = b"[TCP] SMOKE BOOT\n";
const FAIL: &[u8] = b"[TCP] FAIL not-implemented\n";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let _ = mk_debug(BOOT.as_ptr(), BOOT.len());
    let _ = mk_debug(FAIL.as_ptr(), FAIL.len());
    mk_exit(0)
}
