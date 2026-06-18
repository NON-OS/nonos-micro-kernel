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

//! Kernel-side capsule IPC round-trip. The crypto, entropy, and vfs
//! capsules all need the same protocol-agnostic dance: capture the
//! capsule's generation, enqueue the request, drain the reply inbox
//! while re-checking liveness and generation, reject stale or foreign
//! replies. The four wire-shape errors live here as `TransportError`;
//! per-capsule error types map them through `.map_err`.

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use spin::{Mutex, MutexGuard};

use super::state::CapsuleState;
use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox;

const RECV_YIELDS: u32 = 50_000;

/// Acquire a per-capsule transport lock without busy-spinning. `round_trip`
/// yields the CPU while it waits for the reply, so the holder is off-core for
/// the whole exchange; a contending caller must yield too rather than spin, or
/// on a single CPU with interrupts disabled it pins the core and the holder
/// can never run to release the lock (the deadlock involuntary preemption now
/// makes reachable). On contention, hand the CPU to the holder and retry.
pub fn lock_yielding(lock: &'static Mutex<()>) -> MutexGuard<'static, ()> {
    loop {
        if let Some(guard) = lock.try_lock() {
            return guard;
        }
        crate::sched::yield_now();
    }
}

/// Bump a per-capsule request-id counter, skipping zero so callers can
/// reserve `0` for "no request in flight". The atomic is owned by
/// each capsule's `seq` module so increments do not contend across
/// capsules.
pub fn next_request_id(seq: &AtomicU32) -> u32 {
    let v = seq.fetch_add(1, Ordering::Relaxed);
    if v == 0 {
        seq.fetch_add(1, Ordering::Relaxed)
    } else {
        v
    }
}

/// Wire header for the v1 capsule framing. 20 bytes, little-endian:
/// magic[0..4], version[4..6], op[6..8], flags[8..10], reserved[10..12],
/// request_id[12..16], payload_len[16..20]. The crypto, entropy, and
/// vfs capsules all use this layout; each picks its own `magic` and
/// `max_payload`.
pub const FRAME_HDR_LEN: usize = 20;

pub fn encode_request(
    magic: u32,
    version: u16,
    op: u16,
    flags: u16,
    request_id: u32,
    body: &[u8],
) -> Vec<u8> {
    let payload_len = body.len() as u32;
    let mut out = Vec::with_capacity(FRAME_HDR_LEN + body.len());
    out.extend_from_slice(&magic.to_le_bytes());
    out.extend_from_slice(&version.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&flags.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&request_id.to_le_bytes());
    out.extend_from_slice(&payload_len.to_le_bytes());
    out.extend_from_slice(body);
    out
}

pub fn decode_v1_response<'a>(
    buf: &'a [u8],
    magic: u32,
    version: u16,
    max_payload: u32,
) -> Option<DecodedResponse<'a>> {
    if buf.len() < FRAME_HDR_LEN + 4 {
        return None;
    }
    let m = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    if m != magic {
        return None;
    }
    let v = u16::from_le_bytes([buf[4], buf[5]]);
    if v != version {
        return None;
    }
    let op = u16::from_le_bytes([buf[6], buf[7]]);
    let request_id = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]);
    if payload_len > max_payload + 4 {
        return None;
    }
    let total = FRAME_HDR_LEN.saturating_add(payload_len as usize);
    if buf.len() < total || (payload_len as usize) < 4 {
        return None;
    }
    let status = i32::from_le_bytes([
        buf[FRAME_HDR_LEN],
        buf[FRAME_HDR_LEN + 1],
        buf[FRAME_HDR_LEN + 2],
        buf[FRAME_HDR_LEN + 3],
    ]);
    let body = &buf[FRAME_HDR_LEN + 4..total];
    Some(DecodedResponse { op, request_id, status, body })
}

pub struct DecodedResponse<'a> {
    pub op: u16,
    pub request_id: u32,
    pub status: i32,
    pub body: &'a [u8],
}

pub struct ResponseBytes {
    pub status: i32,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    Dead,
    Stale,
    TransportFailure,
    ProtocolMismatch,
}

pub fn round_trip(
    request_id: u32,
    request: &[u8],
    sender_name: &str,
    reply_inbox: &str,
    state: &'static CapsuleState,
    decode: fn(&[u8]) -> Option<DecodedResponse<'_>>,
) -> Result<ResponseBytes, TransportError> {
    let gen_at_send = state.generation();
    if !state.is_alive() {
        return Err(TransportError::Dead);
    }
    let target = format!("proc.{}", state.pid());
    let msg = IpcMessage::new(sender_name, &target, request)
        .map_err(|_| TransportError::TransportFailure)?;
    match nonos_inbox::try_enqueue_strict(&target, msg) {
        Ok(()) => {
            let owner = state.pid();
            if crate::sched::is_sleeping(owner) {
                crate::sched::wake_process(owner);
            }
        }
        // Owner exited between the liveness check above and the
        // enqueue, or the inbox was already unregistered. Either
        // way the capsule is gone from this caller's view; surface
        // it as `Dead` so client code maps to the same errno path.
        Err(nonos_inbox::StrictEnqueueError::MissingInbox)
        | Err(nonos_inbox::StrictEnqueueError::DeadOwner) => {
            return Err(TransportError::Dead);
        }
        Err(nonos_inbox::StrictEnqueueError::QueueFull(_)) => {
            return Err(TransportError::TransportFailure);
        }
    }

    for _ in 0..RECV_YIELDS {
        if !state.is_alive() {
            return Err(TransportError::Dead);
        }
        if state.generation() != gen_at_send {
            return Err(TransportError::Stale);
        }
        if let Some(reply) = nonos_inbox::try_dequeue_existing(reply_inbox) {
            if state.generation() != gen_at_send {
                return Err(TransportError::Stale);
            }
            let resp = decode(&reply.data).ok_or(TransportError::ProtocolMismatch)?;
            if resp.request_id != request_id {
                continue;
            }
            return Ok(ResponseBytes { status: resp.status, body: resp.body.to_vec() });
        }
        crate::sched::yield_now();
    }
    Err(TransportError::TransportFailure)
}
