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

use nonos_libc::{mk_irq_ack, mk_irq_poll, IrqPollOut};

use crate::setup::Driver;

pub(super) fn poll_irq(driver: &Driver, last: &mut u64) {
    let mut irq = IrqPollOut { seq: 0, overflow: 0 };
    if mk_irq_poll(driver.handles.irq_grant_id(), &mut irq as *mut _) >= 0 && irq.seq != *last {
        *last = irq.seq;
        let _ = mk_irq_ack(driver.handles.irq_grant_id());
    }
}
