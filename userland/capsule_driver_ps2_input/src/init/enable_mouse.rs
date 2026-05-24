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
use nonos_libc::{mk_pio_read, mk_pio_write};
use crate::constants::{
    CONFIG_AUX_DISABLE, CONFIG_IRQ1, CONFIG_IRQ12, CTL_ENABLE_AUX, CTL_READ_CONFIG, CTL_WRITE_AUX,
    CTL_WRITE_CONFIG, DATA_OFFSET, MOUSE_ACK, MOUSE_ENABLE_REPORTING, MOUSE_SET_DEFAULTS,
    STATUS_INPUT_FULL, STATUS_OFFSET, STATUS_OUTPUT_FULL,
};
const WAIT_SPINS: u32 = 10_000;
pub fn enable_mouse(grant_id: u64) -> Result<(), &'static str> {
    command(grant_id, CTL_ENABLE_AUX)?;
    command(grant_id, CTL_READ_CONFIG)?;
    let cfg = read_data(grant_id)?;
    command(grant_id, CTL_WRITE_CONFIG)?;
    write_data(grant_id, (cfg | CONFIG_IRQ1 | CONFIG_IRQ12) & !CONFIG_AUX_DISABLE)?;
    mouse_command(grant_id, MOUSE_SET_DEFAULTS)?;
    mouse_command(grant_id, MOUSE_ENABLE_REPORTING)?;
    Ok(())
}
fn mouse_command(grant_id: u64, cmd: u8) -> Result<(), &'static str> {
    command(grant_id, CTL_WRITE_AUX)?;
    write_data(grant_id, cmd)?;
    if read_data(grant_id)? == MOUSE_ACK {
        Ok(())
    } else {
        Err("mouse command not acknowledged")
    }
}
fn command(grant_id: u64, cmd: u8) -> Result<(), &'static str> {
    wait_input_clear(grant_id)?;
    write(grant_id, STATUS_OFFSET, cmd)
}
fn write_data(grant_id: u64, value: u8) -> Result<(), &'static str> {
    wait_input_clear(grant_id)?;
    write(grant_id, DATA_OFFSET, value)
}
fn read_data(grant_id: u64) -> Result<u8, &'static str> {
    wait_output_full(grant_id)?;
    let mut value = 0u32;
    if mk_pio_read(grant_id, DATA_OFFSET, 1, &mut value) < 0 {
        return Err("ps2 data read failed");
    }
    Ok(value as u8)
}
fn wait_input_clear(grant_id: u64) -> Result<(), &'static str> {
    for _ in 0..WAIT_SPINS {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err("ps2 status read failed");
        }
        if status as u8 & STATUS_INPUT_FULL == 0 {
            return Ok(());
        }
    }
    Err("ps2 input buffer busy")
}
fn wait_output_full(grant_id: u64) -> Result<(), &'static str> {
    for _ in 0..WAIT_SPINS {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err("ps2 status read failed");
        }
        if status as u8 & STATUS_OUTPUT_FULL != 0 {
            return Ok(());
        }
    }
    Err("ps2 output buffer empty")
}
fn write(grant_id: u64, offset: u16, value: u8) -> Result<(), &'static str> {
    if mk_pio_write(grant_id, offset, 1, value as u32) < 0 {
        Err("ps2 write failed")
    } else {
        Ok(())
    }
}