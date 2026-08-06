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

use super::mixnet_frame::{self, MAX_BODY};
use crate::clients::nym;
use crate::protocol::{E_BAD_LEN, E_NO_TRANSPORT, E_OK};
use crate::state;

/// Send a write over the mixnet, a frame at a time.
///
/// A mix packet is a fixed size, so a write larger than one frame body has to
/// become several. Refusing instead capped a single write at just under a
/// kilobyte, which no TLS record fits in, so nothing that negotiates a
/// connection could be sent at all.
///
/// Splitting one write across frames is the same thing the caller writing
/// twice already does, so it asks nothing of the far end that the protocol
/// did not already require.
pub fn send_mixnet(handle: u32, ip: [u8; 4], port: u16, payload: &[u8]) -> u16 {
    // An empty write still carries meaning as a frame, so it is not skipped.
    if payload.is_empty() {
        return send_frame(handle, ip, port, payload);
    }
    let mut sent = 0usize;
    while sent < payload.len() {
        let end = (sent + MAX_BODY).min(payload.len());
        let errno = send_frame(handle, ip, port, &payload[sent..end]);
        if errno != E_OK {
            return errno;
        }
        sent = end;
    }
    E_OK
}

fn send_frame(handle: u32, ip: [u8; 4], port: u16, body: &[u8]) -> u16 {
    let Some(frame) = mixnet_frame::encode(ip, port, body) else {
        return E_BAD_LEN;
    };
    match nym::send(state::nym(), handle, &frame) {
        Ok(()) => E_OK,
        Err(_) => E_NO_TRANSPORT,
    }
}
