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

mod client;
mod lifecycle;
mod siphash;
mod wait;

use nonos_libc::{heap_init, mk_debug, mk_exit};

const ISS_KEY: [u64; 2] = [0x0706_0504_0302_0100, 0x0f0e_0d0c_0b0a_0908];
const ISS_KAT: u64 = 0xa129_ca61_49be_45e5;

fn iss_ok() {
    let data: [u8; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    if siphash::siphash24(ISS_KEY, &data) == ISS_KAT {
        let m = b"[TCP] ISS-OK\n";
        let _ = mk_debug(m.as_ptr(), m.len());
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    iss_ok();
    lifecycle::run();
    mk_exit(0)
}
