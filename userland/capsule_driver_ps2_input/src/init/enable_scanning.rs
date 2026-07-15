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
use crate::constants::{
    DATA_OFFSET, KBD_ENABLE_SCANNING, STATUS_INPUT_FULL, STATUS_OFFSET, STATUS_OUTPUT_FULL,
};
use nonos_libc::{mk_pio_read, mk_pio_write};

const WAIT_SPINS: u32 = 10_000;

pub fn enable_scanning(grant_id: u64) -> Result<(), &'static str> {
    let mut spins = 0u32;
    while spins < WAIT_SPINS {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err("kbd status read failed");
        }
        if status as u8 & STATUS_INPUT_FULL == 0 {
            if mk_pio_write(grant_id, DATA_OFFSET, 1, KBD_ENABLE_SCANNING as u32) < 0 {
                return Err("kbd enable-scanning write failed");
            }
            consume_ack(grant_id);
            return Ok(());
        }
        spins += 1;
    }
    Err("kbd input buffer busy")
}

// The keyboard answers 0xF4 with an ACK (0xFA). It must be consumed here: the
// mouse bring-up that follows reads the controller config byte through the same
// output buffer, and an ACK still sitting there gets latched as the config
// value, whose write-back sets the port-1 clock-disable bit and kills the
// keyboard. A missing ACK is tolerated (dead device); an unconsumed one is not.
fn consume_ack(grant_id: u64) {
    for _ in 0..WAIT_SPINS {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return;
        }
        if status as u8 & STATUS_OUTPUT_FULL != 0 {
            let mut data = 0u32;
            let _ = mk_pio_read(grant_id, DATA_OFFSET, 1, &mut data);
            return;
        }
        core::hint::spin_loop();
    }
}
