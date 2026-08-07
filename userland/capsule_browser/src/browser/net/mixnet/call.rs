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

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_ipc_call_timeout;

/// A mixnet round trip crosses several hops before an exit answers, so this
/// is generous next to a direct socket call. It is what a call carrying bytes
/// out of the browser is allowed to wait for.
const CALL_MS: u64 = 15_000;

/// What a call carrying nothing may wait for.
///
/// Asking whether more has arrived yet is not the same as sending, and it
/// happens on the thread that draws the window and answers the pointer.
/// Waiting the sending timeout for an answer that has not arrived stops the
/// whole application: the reader cannot move the window, reach a menu or
/// stop the page, and the browser looks like it has crashed when it is in
/// fact waiting patiently. A poll comes back almost at once and the fetch
/// carries on over the ticks that follow.
const POLL_MS: u64 = 60;

/// Largest single answer worth taking from the proxy.
const REPLY_MAX: usize = 36 * 1024;

/// Hand raw SOCKS bytes to `net.socks5` and return whatever it says back.
///
/// The proxy speaks the protocol as bytes rather than as a framed request, so
/// nothing is wrapped here: what the browser would have written to a socket is
/// exactly what is sent.
pub fn exchange(socks_port: u32, data: &[u8]) -> Result<Vec<u8>, ()> {
    // A frame with nothing but its marker is a poll. It carries no bytes for
    // the exit, so there is nothing to wait on the network for.
    let wait = if data.len() <= 1 { POLL_MS } else { CALL_MS };
    let mut rx = vec![0u8; REPLY_MAX];
    let n = mk_ipc_call_timeout(
        socks_port as u64,
        data.as_ptr(),
        data.len(),
        rx.as_mut_ptr(),
        rx.len(),
        wait,
    );
    if n < 0 {
        return Err(());
    }
    rx.truncate(n as usize);
    Ok(rx)
}
