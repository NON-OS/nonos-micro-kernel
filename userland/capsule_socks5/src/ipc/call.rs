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

use super::wire::{decode, encode, HDR_LEN};
use crate::setup::nym_port;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use nonos_libc::mk_ipc_call_timeout;

/// Longest reply worth accepting from the mixnet capsule.
/// Largest answer worth taking from the mixnet capsule in one read. It has
/// to be at least what that capsule will hand over at once, or a long reply
/// is refused here after surviving the whole network.
const REPLY_MAX: usize = 34 * 1024;

/// How long to wait before giving up on the mixnet capsule.
///
/// Bounded rather than open ended: this runs on the path serving a SOCKS
/// client, and a transport that never answers would otherwise hang the
/// connection rather than fail it.
///
/// Wide enough for what the call actually does. A send builds a Sphinx packet
/// and writes it to a gateway across the internet, so the budget covers a
/// round trip and the crypto, not just an IPC hop. At five seconds this
/// expired before the mixnet capsule could answer, and the give up surfaced
/// as a transport failure carrying no reason.
const TIMEOUT_MS: u64 = 15_000;

static NEXT_REQUEST: AtomicU32 = AtomicU32::new(1);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CallError {
    NoTransport,
    Encode,
    Transport,
    Malformed,
    Remote(u16),
}

/// One request to net.nym, returning the reply payload.
pub fn call(op: u16, payload: &[u8]) -> Result<Vec<u8>, CallError> {
    let port = nym_port();
    if port == 0 {
        return Err(CallError::NoTransport);
    }
    let request_id = NEXT_REQUEST.fetch_add(1, Ordering::Relaxed);
    let mut tx = vec![0u8; HDR_LEN + payload.len()];
    let n = encode(op, request_id, payload, &mut tx).ok_or(CallError::Encode)?;
    let mut rx = vec![0u8; REPLY_MAX];
    let got =
        mk_ipc_call_timeout(port as u64, tx.as_ptr(), n, rx.as_mut_ptr(), rx.len(), TIMEOUT_MS);
    if got <= 0 {
        return Err(CallError::Transport);
    }
    let (errno, body) = decode(&rx[..got as usize]).ok_or(CallError::Malformed)?;
    if errno != 0 {
        return Err(CallError::Remote(errno));
    }
    Ok(body.to_vec())
}
