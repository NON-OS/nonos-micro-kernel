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

mod conn;
mod ipc;
mod manager;
mod nym;
mod server;
mod setup;
mod tunnel;
mod wire;

use nonos_libc::{heap_init, mk_exit, mk_ipc_recv};

const OWN_INBOX: u64 = 0;
const RETRY_BACKOFF_MS: u64 = 250;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    wait_for_setup();
    server::run();
}

/// Wait until `net.nym` is up.
///
/// A timed receive parks this capsule off the run queue rather than yielding,
/// which would keep it permanently runnable and burn a core for the life of
/// the boot. Nothing can be served before the mixnet transport exists, so a
/// request arriving inside the window is no worse off than it is today.
fn wait_for_setup() {
    let mut idle = [0u8; 1];
    loop {
        if setup::run().is_ok() {
            return;
        }
        let _ = mk_ipc_recv(OWN_INBOX, idle.as_mut_ptr(), idle.len(), RETRY_BACKOFF_MS);
    }
}
