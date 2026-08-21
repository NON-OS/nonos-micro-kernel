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
mod device;
mod discover;
mod driver;
mod init;
mod protocol;
mod regs;
mod server;
mod setup;
mod state;
mod virgl;
use nonos_libc::{heap_init, mk_exit, mk_service_register, mk_time_millis, mk_yield};

const SERVICE_NAME: &[u8] = b"driver.virtio_gpu0";
const SERVICE_PORT: u32 = 4226;
// Give the device a bounded window to appear, then exit cleanly. On hardware
// with no virtio-gpu (real hardware presents through the GOP framebuffer) it would
// otherwise retry forever; degrading to a clean exit frees the slot and lets
// the compositor fall back.
const PROBE_DEADLINE_MS: i64 = 10_000;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let start = mk_time_millis();
    let driver = loop {
        match setup::run() {
            Ok(driver) => break driver,
            Err(e) => {
                let _ = e;
                if mk_time_millis().wrapping_sub(start) > PROBE_DEADLINE_MS {
                    mk_exit(0);
                }
                for _ in 0..64 {
                    mk_yield();
                }
            }
        }
    };
    if mk_service_register(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), SERVICE_PORT) < 0 {
        mk_exit(1);
    }
    server::run(driver);
}
