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
use crate::controller::{issue_control_transfer, ControlRequest};
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;

pub(super) fn do_transfer(ctx: &mut Context, slot: u8, req: ControlRequest) -> XhciResult<()> {
    let res = ctx
        .driver
        .slots
        .resources_mut(slot, ctx.driver.layout.max_slots)
        .ok_or(XhciError::ControllerUnsupported)?;
    issue_control_transfer(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.event_ring,
        &mut res.ep0,
        slot,
        req,
    )
}
