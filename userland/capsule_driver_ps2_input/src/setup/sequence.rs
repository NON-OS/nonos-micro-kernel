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
use nonos_libc::{mk_device_release, mk_irq_unbind, mk_pio_release};
use super::claim::claim;
use super::driver::Driver;
use super::irq::{bind as irq_bind, bind_raw as irq_bind_raw};
use super::pio::grant as pio_grant;
use crate::discover::{find_ps2_aux, find_ps2_kbd};
use crate::init::{enable_mouse, enable_scanning, flush_output};
pub fn run() -> Result<Driver, &'static str> {
    let dev = find_ps2_kbd().ok_or("ps2 keyboard not present in device list")?;
    let claim_epoch = claim(dev.device_id)?;
    let pio = pio_grant(dev.device_id, claim_epoch)?;
    let irq = irq_bind(dev, claim_epoch, pio.grant_id)?;
    let aux = find_ps2_aux().ok_or("ps2 aux irq not present in device list")?;
    let aux_epoch = match claim(aux.device_id) {
        Ok(epoch) => epoch,
        Err(e) => {
            rollback_primary(dev.device_id, pio.grant_id, irq.grant_id);
            return Err(e);
        }
    };
    let aux_irq = match irq_bind_raw(aux, aux_epoch) {
        Ok(out) => out,
        Err(e) => {
            let _ = mk_device_release(aux.device_id);
            rollback_primary(dev.device_id, pio.grant_id, irq.grant_id);
            return Err(e);
        }
    };
    flush_output(pio.grant_id);
    enable_scanning(pio.grant_id)?;
    let mouse_enabled = enable_mouse(pio.grant_id).is_ok();
    Ok(Driver {
        pio_grant_id: pio.grant_id,
        irq_grant_id: irq.grant_id,
        aux_irq_grant_id: aux_irq.grant_id,
        mouse_enabled,
    })
}
fn rollback_primary(device_id: u64, pio_grant_id: u64, irq_grant_id: u64) {
    let _ = mk_irq_unbind(irq_grant_id);
    let _ = mk_pio_release(pio_grant_id);
    let _ = mk_device_release(device_id);
}