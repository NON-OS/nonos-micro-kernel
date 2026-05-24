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
use nonos_libc::{mk_device_release, mk_irq_bind, IrqBindOut};
use super::mmio::RegisterGrant;
use crate::discover::Found;
pub fn bind(
    dev: Found,
    claim_epoch: u64,
    registers: RegisterGrant,
) -> Result<IrqBindOut, &'static str> {
    let mut out = IrqBindOut { grant_id: 0, vector: 0 };
    if dev.irq_line == 0 || dev.irq_line == 0xFF {
        return Ok(out);
    }
    let r = mk_irq_bind(dev.device_id, claim_epoch, dev.irq_line as u32, 0, 0, &mut out);
    if r < 0 {
        if !registers.release() {
            return Err("irq bind failed; register rollback also failed");
        }
        if mk_device_release(dev.device_id) < 0 {
            return Err("irq bind failed; device release rollback also failed");
        }
        return Err("irq bind failed");
    }
    Ok(out)
}
