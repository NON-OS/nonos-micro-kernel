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

use nonos_libc::{mk_debug, mk_exit, mk_syscall_raw, mk_time_millis};

const BAD_TAG: i64 = 0x2144_4142;
const MDBG_TAG: i64 = 0x4742_444d;
const PASS: &[u8] = b"[SYSCALL-PROOF] PASS 100k invalid-number invalid-pointer invalid-size\n";
const FAIL_LOOP: &[u8] = b"[SYSCALL-PROOF] FAIL loop\n";
const FAIL_NUMBER: &[u8] = b"[SYSCALL-PROOF] FAIL invalid-number\n";
const FAIL_POINTER: &[u8] = b"[SYSCALL-PROOF] FAIL invalid-pointer\n";
const FAIL_SIZE: &[u8] = b"[SYSCALL-PROOF] FAIL invalid-size\n";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    for _ in 0..100000 {
        if mk_time_millis() < 0 {
            let _ = mk_debug(FAIL_LOOP.as_ptr(), FAIL_LOOP.len());
            mk_exit(1);
        }
    }
    if mk_syscall_raw(BAD_TAG, [0; 6]) != -38 {
        let _ = mk_debug(FAIL_NUMBER.as_ptr(), FAIL_NUMBER.len());
        mk_exit(2);
    }
    if mk_syscall_raw(MDBG_TAG, [1, 8, 0, 0, 0, 0]) != -14 {
        let _ = mk_debug(FAIL_POINTER.as_ptr(), FAIL_POINTER.len());
        mk_exit(3);
    }
    if mk_syscall_raw(MDBG_TAG, [PASS.as_ptr() as u64, 257, 0, 0, 0, 0]) != -22 {
        let _ = mk_debug(FAIL_SIZE.as_ptr(), FAIL_SIZE.len());
        mk_exit(4);
    }
    let _ = mk_debug(PASS.as_ptr(), PASS.len());
    mk_exit(0)
}
