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

use nonos_libc::mk_yield;

use super::settle::settle;
use crate::i2c_client::write_read;

// A device may take tens of milliseconds to come out of RESET before it
// raises the zero-length reset sentinel on the input register. Poll that
// register with yields between reads, over a budget long enough to cover the
// ~60ms a slow touchpad needs, rather than a single blind settle.
const RESET_POLL_ATTEMPTS: u32 = 64;
const RESET_POLL_YIELDS: u32 = 2048;

// Read the input register until the zero-length reset report appears, bounding
// the total wait so a missing device cannot hang the driver. Returns true when
// the reset sentinel was observed.
pub(super) fn await_reset(port: u32, addr: u8, input_reg: u16) -> bool {
    if input_reg == 0 {
        settle();
        return false;
    }
    let reg = input_reg.to_le_bytes();
    for _ in 0..RESET_POLL_ATTEMPTS {
        let mut drain = [0u8; 2];
        // Spec first: after reset the device auto-points at the input register
        // and the sentinel comes from a bare read. Devices that only answer a
        // register-addressed read get the fallback.
        let n = match write_read(port, addr, &[], &mut drain) {
            Some(n) if n >= 2 => Some(n),
            _ => write_read(port, addr, &reg, &mut drain),
        };
        if let Some(n) = n {
            // A zero-length report is encoded as a 0x0000 length prefix.
            if n >= 2 && u16::from_le_bytes([drain[0], drain[1]]) == 0 {
                return true;
            }
        }
        for _ in 0..RESET_POLL_YIELDS {
            mk_yield();
        }
    }
    false
}
