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
// Retired syscall tags: CryptoSign, DebugLog, AdminModLoad, and
// GraphicsSurfaceCreate. They were removed from the ABI and must now
// resolve as unknown tags and return -ENOSYS like any other dead number.
const RETIRED: [i64; 4] = [0x4E47_5343, 0x474F_4C44, 0x444F_4D41, 0x5243_5347];
const PASS: &[u8] =
    b"[SYSCALL-PROOF] PASS time-loop invalid-number invalid-pointer invalid-size retired-enosys\n";
const FAIL_LOOP: &[u8] = b"[SYSCALL-PROOF] FAIL loop\n";
const FAIL_NUMBER: &[u8] = b"[SYSCALL-PROOF] FAIL invalid-number\n";
const FAIL_POINTER: &[u8] = b"[SYSCALL-PROOF] FAIL invalid-pointer\n";
const FAIL_SIZE: &[u8] = b"[SYSCALL-PROOF] FAIL invalid-size\n";
const FAIL_RETIRED: &[u8] = b"[SYSCALL-PROOF] FAIL retired-enosys\n";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    for _ in 0..1024 {
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
    let mut i = 0;
    while i < RETIRED.len() {
        if mk_syscall_raw(RETIRED[i], [0; 6]) != -38 {
            let _ = mk_debug(FAIL_RETIRED.as_ptr(), FAIL_RETIRED.len());
            mk_exit(5);
        }
        i += 1;
    }
    let _ = mk_debug(PASS.as_ptr(), PASS.len());
    mk_exit(0)
}
