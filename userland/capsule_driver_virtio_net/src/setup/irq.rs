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

use nonos_libc::{mk_device_release, mk_irq_bind, IrqBindOut, MK_IRQ_BIND_MSIX};

use super::registers::RegisterGrant;
use crate::discover::Found;

pub fn bind(
    dev: Found,
    claim_epoch: u64,
    reg: &RegisterGrant,
) -> Result<(IrqBindOut, bool), &'static str> {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    let r = mk_irq_bind(dev.device_id, claim_epoch, dev.irq_line as u32, 0, 0, &mut out);
    if r >= 0 {
        return Ok((out, false));
    }
    let msix = mk_irq_bind(dev.device_id, claim_epoch, 0, MK_IRQ_BIND_MSIX, 1, &mut out);
    if msix >= 0 {
        return Ok((out, true));
    }
    let reg_ok = reg.release();
    let device_ok = mk_device_release(dev.device_id) >= 0;
    if !reg_ok || !device_ok {
        return Err("irq bind rollback failed");
    }
    Err("irq bind failed")
}
