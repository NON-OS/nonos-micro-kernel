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
use crate::contexts::write_configure_endpoint_input;
use crate::controller::issue_configure_endpoint;
use crate::error::{XhciError, XhciResult};
use crate::protocol::HID_REPORT_MAX;
use crate::rings::transfer::TransferRing;
use crate::server::context::Context;

pub(super) fn do_configure(
    ctx: &mut Context,
    slot: u8,
    dci: u8,
    max_packet: u16,
    interval: u8,
) -> XhciResult<()> {
    let ring = TransferRing::new(&ctx.driver.dma_pool)?;
    let ring_phys = ring.phys();
    let buf = ctx.driver.dma_pool.alloc(HID_REPORT_MAX as u64)?;
    let input_ctx_phys = {
        let res = ctx
            .driver
            .slots
            .resources_mut(slot, ctx.driver.layout.max_slots)
            .ok_or(XhciError::ControllerUnsupported)?;
        res.int_ring = Some(ring);
        res.int_buf = Some(buf);
        res.int_dci = dci;
        res.int_armed = None;
        write_configure_endpoint_input(
            &res.input_context,
            crate::contexts::EndpointConfig {
                context_size: ctx.driver.layout.context_size,
                dci,
                ring_phys,
                max_packet,
                interval,
                speed: res.speed,
                root_port: res.port_id,
            },
        );
        res.input_context.phys()
    };
    issue_configure_endpoint(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.command_ring,
        &mut ctx.driver.event_ring,
        input_ctx_phys,
        slot,
    )
}
