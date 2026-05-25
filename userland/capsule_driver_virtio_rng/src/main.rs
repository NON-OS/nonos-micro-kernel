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
mod debug;
mod discover;
mod fill;
mod init;
mod protocol;
mod queue;
mod regs;
mod server;
mod setup;

use nonos_libc::{heap_init, mk_exit, mk_yield};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    debug::marker(b"heap ok");

    let mut driver = loop {
        match setup::run() {
            Ok(d) => {
                debug::marker(b"setup ok");
                break d
            }
            Err(_) => {
                debug::marker(b"setup err");
                for _ in 0..64 {
                    mk_yield();
                    debug::marker(b"retry ok");
                }
            }
        }
    };

    // Sanity check: pull one fill before opening the service.
    // A device that fails the first round trip is not worth
    // exposing as an entropy source.
    match crate::fill::fill(driver.regs, &mut driver.queue, driver.irq_grant) {
        Ok(n) => {
            debug::marker(b"fill ok");
            let bytes = driver.queue.buffer(n);
            let mut nz = 0usize;
            for &b in bytes.iter() {
                if b != 0 {
                    nz += 1;
                }
            }
            if nz == 0 {
                debug::marker(b"fill zero");
                driver.release();
                mk_exit(4);
            }
        }
        Err(_) => {
            debug::marker(b"fill err");
            driver.release();
            mk_exit(3);
        }
    }

    let _ = driver.queue.region_phys();
    let _ = driver.claim_epoch;
    debug::marker(b"server enter");
    server::run(&mut driver);
}
