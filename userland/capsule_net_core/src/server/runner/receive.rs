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

use core::sync::atomic::AtomicU32;
use nonos_libc::mk_ipc_recv_from;

use super::cadence::{next_wait, note_idle, note_work};

const SERVICE_INBOX: u64 = 0;

/// Ticks of quiet left before the loop goes back to waiting patiently.
static BUSY: AtomicU32 = AtomicU32::new(0);

/// Wait for a request, for as long as the connection can afford.
///
/// The wait is also how long an arriving packet sits in the card before
/// anything looks at it, because the device is polled when this returns. A
/// fixed fifty milliseconds is cheap while nothing is happening and very
/// expensive during a handshake, which is several round trips that each pay
/// it once. So the loop waits briefly while there is traffic and settles
/// back down when there is none.
pub fn receive(rx: &mut [u8], sender_pid: &mut u32) -> i64 {
    let wait = next_wait(&BUSY);
    let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, sender_pid);
    if n > 0 {
        note_work(&BUSY);
    } else {
        note_idle(&BUSY);
    }
    n
}
