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
use super::alloc_resources::alloc_resources;
use super::complete_address::complete_address;
use crate::protocol::{Request, E_IO, E_NODEV};
use crate::server::context::Context;
use crate::server::error::reply_with_status;

pub fn address_after_reset(
    ctx: &mut Context,
    req: &Request,
    tx: &mut [u8],
    slot: u8,
    port: u8,
    speed: u8,
) {
    if speed == 0 {
        reply_with_status(tx, req, E_NODEV);
        return;
    }
    let resources = match alloc_resources(ctx, slot, port, speed) {
        Ok(r) => r,
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    complete_address(ctx, req, tx, resources);
}
