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
use crate::controller::{clear_dcbaa_slot, issue_disable_slot};
use crate::protocol::{Request, E_INVAL, E_IO, SLOT_DISABLE_PAYLOAD_LEN};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != SLOT_DISABLE_PAYLOAD_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let slot_id = body[0];
    if !ctx.driver.slots.is_allocated(slot_id, ctx.driver.layout.max_slots) {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let addressed = ctx.driver.slots.is_addressed(slot_id, ctx.driver.layout.max_slots);
    if issue_disable_slot(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.command_ring,
        &mut ctx.driver.event_ring,
        slot_id,
    )
    .is_err()
    {
        reply_with_status(tx, req, E_IO);
        return;
    }
    if addressed {
        let _ = clear_dcbaa_slot(&ctx.driver.dcbaa, slot_id, ctx.driver.layout.max_slots);
        let _ = ctx.driver.slots.take_resources(slot_id, ctx.driver.layout.max_slots);
    }
    let _ = ctx.driver.slots.mark_released(slot_id, ctx.driver.layout.max_slots);
    reply_with_status(tx, req, 0);
}
