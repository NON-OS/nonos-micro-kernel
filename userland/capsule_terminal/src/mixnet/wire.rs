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

/// A mixnet round trip crosses several hops before an exit answers, so this is
/// generous next to a direct socket call.
const CALL_MS: u64 = 15_000;
const REPLY_MAX: usize = 8192;

/// Hand raw SOCKS bytes to `net.socks5` and return what it says back.
///
/// The proxy speaks the protocol as bytes, so nothing is wrapped: what would
/// have been written to a socket is exactly what is sent.
pub fn exchange(socks_port: u32, data: &[u8]) -> Result<Vec<u8>, ()> {
    let mut rx = vec![0u8; REPLY_MAX];
    let n = mk_ipc_call_timeout(
        socks_port as u64,
        data.as_ptr(),
        data.len(),
        rx.as_mut_ptr(),
        rx.len(),
        CALL_MS,
    );
    if n < 0 {
        return Err(());
    }
    rx.truncate(n as usize);
    Ok(rx)
}
