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

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::error::BlkError;
use super::request::encode_request;

const SERVICE: &[u8] = b"driver.virtio_blk0";

// The driver only reaches its receive loop after PCI discovery, feature
// negotiation and queue setup, so the first request can sit unserved for a
// while; under TCG that startup is ~75x slower than native. Eight seconds
// covers it and still bounds the worst case a wedged driver can add to boot.
const TIMEOUT_MS: u64 = 8000;

static REQUEST_ID: AtomicU32 = AtomicU32::new(1);

// One request/reply round-trip. `mk_ipc_call_timeout` is mandatory here: a bare
// send plus recv on inbox 0 has the reply routed to the wrong inbox and never
// arrives. Returns the received byte count and the id the reply must echo.
pub fn call(op: u16, body: &[u8], rx: &mut [u8]) -> Result<(usize, u32), BlkError> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        return Err(BlkError::NoService);
    }
    let request_id = REQUEST_ID.fetch_add(1, Ordering::Relaxed);
    let tx = encode_request(op, request_id, body);
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < 0 {
        return Err(BlkError::Transport(n));
    }
    Ok((n as usize, request_id))
}
