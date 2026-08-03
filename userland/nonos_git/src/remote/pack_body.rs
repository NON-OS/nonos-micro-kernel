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
//! Finding the pack inside an upload-pack response.

use crate::transport::TransportError;
use crate::wire::{read_pkt, Pkt};

/// Skip the packet lines a server sends before the pack and return the pack.
///
/// A shallow fetch is answered with `shallow` lines and a flush before the
/// acknowledgement, so this walks packets until the acknowledgement rather
/// than assuming a fixed prelude. Everything after it is the pack itself,
/// because the request does not ask for the side-band that would otherwise
/// wrap it.
pub(super) fn pack_body(response: &[u8]) -> Result<&[u8], TransportError> {
    let mut at = 0usize;
    while at < response.len() {
        if response[at..].starts_with(b"PACK") {
            return Ok(&response[at..]);
        }
        let (pkt, used) = read_pkt(&response[at..]).map_err(|_| TransportError::Malformed)?;
        at += used;
        if let Pkt::Data(line) = pkt {
            if line.starts_with(b"NAK") || line.starts_with(b"ACK") {
                return response.get(at..).ok_or(TransportError::Malformed);
            }
        }
    }
    Err(TransportError::Malformed)
}
