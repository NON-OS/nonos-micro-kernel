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

use super::args::Args;
use crate::syscall::microkernel::irq::{
    sys_irq_ack, sys_irq_bind, sys_irq_poll, sys_irq_unbind, sys_irq_wait,
};
use crate::syscall::microkernel::numbers::*;

pub(super) fn handle(nr: u64, a: Args) -> Option<i64> {
    Some(match nr {
        SYS_IRQ_BIND => sys_irq_bind(a.a0, a.a1, a.a2 as u32, a.a3 as u32, a.a4 as u32, a.a5),
        SYS_IRQ_UNBIND => sys_irq_unbind(a.a0),
        SYS_IRQ_ACK => sys_irq_ack(a.a0),
        SYS_IRQ_POLL => sys_irq_poll(a.a0, a.a1),
        SYS_IRQ_WAIT => sys_irq_wait(a.a0, a.a1, a.a2, a.a3),
        _ => return None,
    })
}
