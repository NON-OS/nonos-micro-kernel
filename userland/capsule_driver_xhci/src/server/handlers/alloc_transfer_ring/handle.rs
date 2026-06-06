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
use super::configure::do_configure;
use super::reply::send_reply;
use crate::protocol::{Request, ALLOC_TRANSFER_RING_REQUEST_LEN, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::dci_from_ep_address;

pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != ALLOC_TRANSFER_RING_REQUEST_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let slot = body[0];
    let max_packet = u16::from_le_bytes([body[2], body[3]]);
    let dci = dci_from_ep_address(body[1]);
    match do_configure(ctx, slot, dci, max_packet, body[4]) {
        Ok(()) => send_reply(tx, req, dci),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
