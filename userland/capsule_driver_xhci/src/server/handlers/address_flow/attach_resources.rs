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
use super::super::address_reply::reply_ok;
use crate::controller::clear_dcbaa_slot;
use crate::protocol::{Request, E_INVAL};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::SlotResources;

pub(super) fn attach_resources(
    ctx: &mut Context,
    req: &Request,
    tx: &mut [u8],
    resources: SlotResources,
) {
    let (slot, port, speed, mps) =
        (resources.slot_id, resources.port_id, resources.speed, resources.max_packet);
    if !ctx.driver.slots.attach_addressed(resources, ctx.driver.layout.max_slots) {
        let _ = clear_dcbaa_slot(&ctx.driver.dcbaa, slot, ctx.driver.layout.max_slots);
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    reply_ok(tx, req, slot, port, speed, mps);
}
