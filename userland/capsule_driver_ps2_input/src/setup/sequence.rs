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
use super::irq::bind as irq_bind;
use super::marker::marker;
use super::open_line::open_line;
use super::pio::grant as pio_grant;
use super::setup_aux::setup_aux;
use crate::discover::find_ps2_kbd;
use crate::init::{disable_aux, enable_keyboard, enable_mouse, flush_output};

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
    let (mouse_enabled, mouse_wheel) = if aux_irq_grant_id != 0 {
        match enable_mouse(pio_grant_id) {
            Ok(wheel) => (true, wheel),
            Err(_) => {
                // enable_mouse turned the aux clock on before the step that
                // failed; on a machine with no PS/2 mouse behind the port
                // (firmware-disabled aux) leaving it enabled streams garbage
                // the drain would have to discard forever. Turn it back off.
                disable_aux(pio_grant_id);
                (false, false)
            }
        }
    } else {
        (false, false)
    };
    open_line(irq_grant_id);
    open_line(aux_irq_grant_id);
    marker(b"[driver_ps2] endpoint driver.ps2_kbd0 ready\n");
    Ok(Driver { pio_grant_id, irq_grant_id, aux_irq_grant_id, mouse_enabled, mouse_wheel })
}
