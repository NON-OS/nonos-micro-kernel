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
use crate::protocol::{
    Request, E_INVAL, OP_ADDRESS_DEVICE, OP_ALLOC_TRANSFER_RING, OP_CONTROLLER_STATUS,
    OP_CONTROL_TRANSFER, OP_DISABLE_SLOT, OP_ENABLE_SLOT, OP_GET_CONFIG_DESCRIPTOR,
    OP_GET_DEVICE_DESCRIPTOR, OP_HEALTHCHECK, OP_INTERRUPT_IN, OP_PORT_STATUS,
};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers;

pub fn dispatch(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(req, tx),
        OP_CONTROLLER_STATUS if body.is_empty() => {
            handlers::controller_status::handle(ctx, req, tx)
        }
        OP_PORT_STATUS if body.is_empty() => handlers::port_status::handle(ctx, req, tx),
        OP_ENABLE_SLOT if body.is_empty() => handlers::enable_slot::handle(ctx, req, tx),
        OP_DISABLE_SLOT => handlers::disable_slot::handle(ctx, req, body, tx),
        OP_ADDRESS_DEVICE => handlers::address_device::handle(ctx, req, body, tx),
        OP_GET_DEVICE_DESCRIPTOR => handlers::device_descriptor::handle(ctx, req, body, tx),
        OP_GET_CONFIG_DESCRIPTOR => handlers::config_descriptor::handle(ctx, req, body, tx),
        OP_CONTROL_TRANSFER => handlers::control_transfer::handle(ctx, req, body, tx),
        OP_ALLOC_TRANSFER_RING => handlers::alloc_transfer_ring::handle(ctx, req, body, tx),
        OP_INTERRUPT_IN => handlers::interrupt_in::handle(ctx, req, body, tx),
        _ => reply_with_status(tx, req, E_INVAL),
    }
}
