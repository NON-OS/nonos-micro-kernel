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

//! The round trip every capsule on this machine pays for.
//!
//! Message out, scheduler, server wakes, server replies, scheduler, message
//! back. It is the operation a microkernel is judged on and the one that was
//! never measured, because measuring it needs a peer that answers and the
//! earlier probes only needed the kernel.
//!
//! `attest` is the peer. Its healthcheck reads no state and touches no device,
//! so what gets timed is the transport rather than the work at the far end.
//! A service that did anything would report its own cost as ours.

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::ipc_wire::{HDR_LEN, MAGIC, OP_HEALTHCHECK, SERVICE, TIMEOUT_MS, VERSION};

pub struct Peer {
    port: u32,
    tx: [u8; HDR_LEN],
}

impl Peer {
    /// `None` when the service is not registered, which is a real state on a
    /// profile that ships no attest capsule and must not be reported as a
    /// measurement of zero.
    pub fn find() -> Option<Self> {
        let mut port = 0u32;
        let mut pid = 0u32;
        let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
        if rc != 0 || port == 0 {
            return None;
        }
        let mut tx = [0u8; HDR_LEN];
        tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
        tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
        tx[6..8].copy_from_slice(&OP_HEALTHCHECK.to_le_bytes());
        Some(Peer { port, tx })
    }

    /// One round trip. The reply is read into a buffer the caller owns across
    /// the whole run, so the allocation is not inside the timed section.
    pub fn ping(&self, rx: &mut [u8]) {
        let _ = mk_ipc_call_timeout(
            self.port as u64,
            self.tx.as_ptr(),
            self.tx.len(),
            rx.as_mut_ptr(),
            rx.len(),
            TIMEOUT_MS,
        );
    }
}
