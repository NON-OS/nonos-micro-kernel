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
use crate::constants::{PORTSC_SPEED_MASK, PORTSC_SPEED_SHIFT};
use crate::controller::{clear_dcbaa_slot, issue_address_device, set_dcbaa_slot};
use crate::error::XhciResult;
use crate::protocol::{Request, E_INVAL, E_IO, E_NODEV};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::SlotResources;
use super::address_reply::reply_ok;
pub fn slot_ready(ctx: &Context, slot_id: u8, port_id: u8) -> bool {
    port_id != 0
        && port_id <= ctx.driver.layout.max_ports
        && ctx.driver.slots.is_allocated(slot_id, ctx.driver.layout.max_slots)
        && !ctx.driver.slots.is_addressed(slot_id, ctx.driver.layout.max_slots)
}
pub fn port_speed(portsc: u32) -> u8 {
    ((portsc & PORTSC_SPEED_MASK) >> PORTSC_SPEED_SHIFT) as u8
}
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
fn alloc_resources(ctx: &Context, slot: u8, port: u8, speed: u8) -> XhciResult<SlotResources> {
    SlotResources::allocate(
        &ctx.driver.dma_pool,
        ctx.driver.layout.context_size,
        slot,
        port,
        speed,
    )
}
fn complete_address(ctx: &mut Context, req: &Request, tx: &mut [u8], resources: SlotResources) {
    let slot = resources.slot_id;
    let output_phys = resources.output_context.phys();
    if set_dcbaa_slot(&ctx.driver.dcbaa, slot, ctx.driver.layout.max_slots, output_phys).is_err() {
        reply_with_status(tx, req, E_IO);
        return;
    }
    if command_address(ctx, &resources).is_err() {
        let _ = clear_dcbaa_slot(&ctx.driver.dcbaa, slot, ctx.driver.layout.max_slots);
        reply_with_status(tx, req, E_IO);
        return;
    }
    attach_resources(ctx, req, tx, resources);
}
fn attach_resources(ctx: &mut Context, req: &Request, tx: &mut [u8], resources: SlotResources) {
    let (slot, port, speed, mps) =
        (resources.slot_id, resources.port_id, resources.speed, resources.max_packet);
    if !ctx.driver.slots.attach_addressed(resources, ctx.driver.layout.max_slots) {
        let _ = clear_dcbaa_slot(&ctx.driver.dcbaa, slot, ctx.driver.layout.max_slots);
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    reply_ok(tx, req, slot, port, speed, mps);
}
fn command_address(ctx: &mut Context, resources: &SlotResources) -> XhciResult<()> {
    issue_address_device(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.command_ring,
        &mut ctx.driver.event_ring,
        resources.input_context.phys(),
        resources.slot_id,
    )
}