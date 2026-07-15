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
use crate::constants::{CTL_ENABLE_KBD, STATUS_INPUT_FULL, STATUS_OFFSET};
use nonos_libc::{mk_pio_read, mk_pio_write};

const WAIT_SPINS: u32 = 10_000;

// Controller command 0xAE enables the first PS/2 port, clearing the config
// clock-disable bit in effect. Firmware can hand the machine off with the port
// disabled (a 0xAD issued during its own probe, never undone); on such a box
// nothing else re-enables the port and the keyboard stays silent forever. The
// command produces no response byte and is idempotent on an already-enabled
// port, so it is always safe to send before scanning is turned on.
pub(super) fn enable_port(grant_id: u64) -> Result<(), &'static str> {
    let mut spins = 0u32;
    while spins < WAIT_SPINS {
        let mut status = 0u32;
        if mk_pio_read(grant_id, STATUS_OFFSET, 1, &mut status) < 0 {
            return Err("kbd status read failed");
        }
        if status as u8 & STATUS_INPUT_FULL == 0 {
            if mk_pio_write(grant_id, STATUS_OFFSET, 1, CTL_ENABLE_KBD as u32) < 0 {
                return Err("kbd enable-port write failed");
            }
            return Ok(());
        }
        spins += 1;
    }
    Err("kbd input buffer busy")
}
