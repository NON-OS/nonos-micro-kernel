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
use crate::controller::{ack_irq, drain_events};
use crate::server::context::Context;

pub fn service_interrupts(ctx: &mut Context) {
    let _ = drain_events(ctx.driver.layout.primary_intr_base, &mut ctx.driver.event_ring);
    ack_irq(ctx.driver.layout.primary_intr_base, ctx.driver.handles.irq_grant_id());
}
