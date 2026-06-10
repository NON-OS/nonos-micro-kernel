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
use super::claim::claim;
use super::driver::Driver;
use super::irq::{bind as irq_bind, bind_raw as irq_bind_raw};
use super::pio::grant as pio_grant;
use crate::discover::{find_ps2_aux, find_ps2_kbd};
use crate::init::{enable_keyboard, enable_mouse, flush_output};
use nonos_libc::{mk_device_release, mk_irq_ack};

pub fn run() -> Result<Driver, &'static str> {
    let dev = find_ps2_kbd().ok_or("ps2 keyboard not present in device list")?;
    let claim_epoch = claim(dev.device_id)?;
    let mut pio_grant_id = pio_grant(dev.device_id, claim_epoch)?.grant_id;
    let irq_grant_id = match irq_bind(dev, claim_epoch, pio_grant_id) {
        Ok(out) => out.grant_id,
        Err(_) => {
            pio_grant_id = pio_grant(dev.device_id, claim_epoch)?.grant_id;
            0
        }
    };
    let aux_irq_grant_id = setup_aux();
    flush_output(pio_grant_id);
    enable_keyboard(pio_grant_id)?;
    let mouse_enabled = aux_irq_grant_id != 0 && enable_mouse(pio_grant_id).is_ok();
    open_line(irq_grant_id);
    open_line(aux_irq_grant_id);
    Ok(Driver { pio_grant_id, irq_grant_id, aux_irq_grant_id, mouse_enabled })
}

// `mk_irq_bind` leaves the GSI masked until the capsule signals it
// is ready; without this opening ack the line never delivers and
// the first IRQ can never arrive to make the pump ack it.
fn open_line(grant_id: u64) {
    if grant_id != 0 {
        let _ = mk_irq_ack(grant_id);
    }
}

fn setup_aux() -> u64 {
    let Some(aux) = find_ps2_aux() else {
        return 0;
    };
    let Ok(epoch) = claim(aux.device_id) else {
        return 0;
    };
    match irq_bind_raw(aux, epoch) {
        Ok(out) => out.grant_id,
        Err(_) => {
            let _ = mk_device_release(aux.device_id);
            0
        }
    }
}
