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

mod ack;
mod crypto;
mod directory_sync;
mod gateway_client;
mod json;
mod message;
mod mixnet;
mod packet;
mod payload;
mod protocol;
mod reply;
mod route;
mod server;
mod setup;
mod sphinx;
mod state;
mod surb;
mod tcp_client;
mod topology;
mod trace;

use nonos_libc::{heap_init, mk_exit, mk_ipc_recv};

const OWN_INBOX: u64 = 0;
const RETRY_BACKOFF_MS: u64 = 250;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    wait_for_setup();
    // Publish the routes that shipped in this image. They arrived inside a
    // kernel the bootloader measured and verified, so they are usable without
    // asking anyone to sign them again. A fetched directory replaces this and
    // does have to prove itself.
    let _ = topology::install_builtin();
    // The gateway is reached from inside the serve loop, one candidate per
    // idle moment. Connecting here instead held the capsule for as long as
    // the whole bootstrap list took, and every capsule downstream waits on
    // this one, so the desktop waited with it.
    server::run();
}

// The transport this capsule sits on is absent in every profile that does
// not build it, so this wait has no bound. Retrying through `mk_yield` left
// the capsule permanently runnable, which kept the scheduler off its halt
// path and burned a whole core for the life of the boot. A timed receive
// parks it off the run queue instead; no client can be served until setup
// succeeds, so a request landing inside the window is no more lost than it
// is today.
fn wait_for_setup() {
    let mut idle = [0u8; 1];
    loop {
        if setup::run().is_ok() {
            return;
        }
        let _ = mk_ipc_recv(OWN_INBOX, idle.as_mut_ptr(), idle.len(), RETRY_BACKOFF_MS);
    }
}
