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

use nonos_libc::{mk_uptime_ms, mk_yield};

use super::envelope::call;

const MAGIC: u32 = 0x4E54_4350;
const SEND: u16 = 5;
/// The most net.tcp will take in one request, which is the MSS an Ethernet
/// link gives a segment.
const SEGMENT_MAX: usize = 1460;
/// How long to keep offering bytes a full socket has not taken yet.
const SEND_DEADLINE_MS: i64 = 10_000;
const E_LEN: u16 = 4;
const E_SHORT_WRITE: u16 = 24;

/// Write every byte, offering again whatever the socket did not take.
///
/// A whole buffer used to go over in one request. Anything past what net.tcp
/// can receive in one message was lost before it was parsed, which is what a
/// TLS record or a POST body larger than a segment looks like. The count in
/// the reply used to be discarded too, so a send buffer with partial room
/// dropped the remainder of a stream that cannot ask for it again.
pub fn send(port: u32, handle: u32, payload: &[u8]) -> Result<(), u16> {
    let deadline = mk_uptime_ms().saturating_add(SEND_DEADLINE_MS);
    let mut sent = 0usize;
    while sent < payload.len() {
        let end = (sent + SEGMENT_MAX).min(payload.len());
        let took = send_chunk(port, handle, &payload[sent..end])?;
        if took == 0 {
            if mk_uptime_ms() >= deadline {
                return Err(E_SHORT_WRITE);
            }
            mk_yield();
            continue;
        }
        sent += took;
    }
    Ok(())
}

/// Offer one chunk and report how much of it the socket took.
fn send_chunk(port: u32, handle: u32, chunk: &[u8]) -> Result<usize, u16> {
    let mut body = vec![0u8; 4 + chunk.len()];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..].copy_from_slice(chunk);
    // net.tcp replies with the count it accepted. Room has to be made for it,
    // or a delivered send comes back as a body length mismatch.
    let mut out = [0u8; 4];
    if call(port, MAGIC, SEND, &body, &mut out)? != 4 {
        return Err(E_LEN);
    }
    let took = u32::from_le_bytes(out) as usize;
    if took > chunk.len() {
        return Err(E_LEN);
    }
    Ok(took)
}
