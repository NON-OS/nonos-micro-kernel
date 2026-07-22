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
mod keymap;
mod mouse;
mod poll;
mod protocol;
mod ring;
mod server;
mod setup;
use nonos_libc::{heap_init, mk_exit, mk_time_millis, mk_yield};

// Bounded probe: exit cleanly if no PS/2 controller answers, instead of
// spinning forever on hardware whose keyboard and pointer are USB or i2c.
const PROBE_DEADLINE_MS: i64 = 10_000;

/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh stack
/// with the capsule's heap region reserved; it must never be called from Rust.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let start = mk_time_millis();
    let driver = loop {
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
    server::run(driver);
}
