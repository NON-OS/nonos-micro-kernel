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

mod constants;
mod discover;
mod init;
mod protocol;
mod queue;
mod regs;
mod rx;
mod server;
mod setup;
mod tx;

use nonos_libc::{heap_init, mk_exit, mk_time_millis, mk_yield};

// Bounded probe: exit cleanly if no virtio-net appears, instead of spinning
// forever on hardware that has none (real machines use their physical NIC).
const PROBE_DEADLINE_MS: i64 = 10_000;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }

    let start = mk_time_millis();
    let mut driver = loop {
        match setup::run() {
            Ok(d) => break d,
            Err(_) => {
                if mk_time_millis().wrapping_sub(start) > PROBE_DEADLINE_MS {
                    mk_exit(0);
                }
                for _ in 0..64 {
                    mk_yield();
                }
            }
        }
    };

    if driver.rx.region_phys() == 0 || driver.tx.region_phys() == 0 {
        if !driver.release() {
            mk_exit(4);
        }
        mk_exit(3);
    }
    server::run(&mut driver);
}
