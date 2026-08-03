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

use super::feed::feed;
use super::state::reset;
use alloc::vec;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};

/// Largest SOCKS exchange worth buffering.
const RX_MAX: usize = 4096;

/// Serve SOCKS clients over IPC.
///
/// The client speaks RFC 1928 as bytes; this capsule is the far end of that
/// conversation and the near end of a mixnet tunnel. Nothing here opens a
/// socket, which is what keeps a clearnet path from existing at all.
pub fn run() -> ! {
    reset();
    let mut rx = vec![0u8; RX_MAX];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if n <= 0 || sender == 0 {
            continue;
        }
        let out = feed(&rx[..n as usize]);
        if !out.is_empty() {
            let _ = mk_ipc_reply(sender, out.as_ptr(), out.len());
        }
    }
}
