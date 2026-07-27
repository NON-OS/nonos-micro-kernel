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

use nonos_libc::{mk_irq_bind, IrqBindOut, MK_IRQ_BIND_MSIX};

use super::mark::mark;
use crate::discover::Found;
use crate::error::HdaResult;

pub fn bind(dev: Found, claim_epoch: u64) -> HdaResult<IrqBindOut> {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    let intx = mk_irq_bind(dev.device_id, claim_epoch, dev.irq_line as u32, 0, 0, &mut out);
    if intx >= 0 {
        return Ok(out);
    }
    let msix = mk_irq_bind(dev.device_id, claim_epoch, 0, MK_IRQ_BIND_MSIX, 1, &mut out);
    if msix >= 0 {
        return Ok(out);
    }
    mark("[HDA] irq-polled\n");
    Ok(IrqBindOut { grant_id: 0, vector: 0 })
}
