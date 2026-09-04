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

use nonos_libc::{heap_init, mk_exit, mk_ipc_recv_from, mk_ipc_reply};

const OWN_INBOX: u64 = 0;
const RETRY_BACKOFF_MS: u64 = 250;

/// Enough to take a whole waiting request off the queue while parked. The
/// content is discarded either way; what matters is that the caller gets an
/// answer instead of losing its message into a one-byte buffer.
const PARK_RX: usize = 4096;

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
/// the boot. A request that arrives inside the window is answered with a
/// closed-stream marker rather than dequeued into oblivion: the old one-byte
/// receive destroyed the caller's message and said nothing, so every early
/// page load waited out its whole timeout on an answer that was never coming.
/// A caller told "closed" fails fast and reconnects once serving starts.
fn wait_for_setup() {
    let mut rx = [0u8; PARK_RX];
    loop {
        if setup::run().is_ok() {
            return;
        }
        let mut sender = 0u32;
        let n =
            mk_ipc_recv_from(OWN_INBOX, rx.as_mut_ptr(), rx.len(), RETRY_BACKOFF_MS, &mut sender);
        if n > 0 && sender != 0 {
            let closed = [server::STREAM_CLOSED];
            let _ = mk_ipc_reply(sender, closed.as_ptr(), closed.len());
        }
    }
}
