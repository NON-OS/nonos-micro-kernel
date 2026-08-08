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

use super::{mixnet_frame, mixnet_residual};
use crate::clients::nym;
use crate::protocol::{E_BAD_LEN, E_NOT_CONNECTED, E_NO_TRANSPORT};
use crate::sockets::Socket;
use crate::state;

/// Read from the mixnet, finishing a part read frame before pulling another.
pub fn recv_mixnet(sock: Socket, out: &mut [u8]) -> Result<usize, u16> {
    // Finish the frame the last read was too small for before pulling a new
    // one, or the stream is delivered out of order.
    let held = mixnet_residual::take(sock.key, out);
    if held > 0 {
        return Ok(held);
    }
    let Some(remote) = sock.remote else { return Err(E_NOT_CONNECTED) };
    let mut frame = [0u8; mixnet_frame::MAX_BODY + 16];
    let n =
        nym::recv(state::nym(), sock.transport_handle, &mut frame).map_err(|_| E_NO_TRANSPORT)?;
    let decoded = mixnet_frame::decode(&frame[..n]).ok_or(E_NO_TRANSPORT)?;
    if decoded.ip != remote.ip || decoded.port != remote.port {
        return Err(E_NOT_CONNECTED);
    }
    // The frame is off the queue now, so whatever does not fit has to be kept
    // rather than refused: there is no asking for it again.
    let taken = decoded.body.len().min(out.len());
    out[..taken].copy_from_slice(&decoded.body[..taken]);
    if !mixnet_residual::store(sock.key, &decoded.body[taken..]) {
        return Err(E_BAD_LEN);
    }
    Ok(taken)
}
