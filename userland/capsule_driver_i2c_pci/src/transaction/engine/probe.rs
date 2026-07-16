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
use crate::driver::Driver;
use crate::transaction::{TransferError, TransferRequest, FLAG_RESTART_ON_READ};

use super::transfer::transfer;

// A bare address probe: does the device acknowledge a one-byte read. Kept for the
// generic OP_PROBE service where the caller only wants presence on the bus.
pub fn probe(driver: &Driver, addr: u8) -> Result<bool, TransferError> {
    let req = TransferRequest { addr, flags: 0, write: &[], read_len: 1 };
    match transfer(driver, req) {
        Ok(_) => Ok(true),
        Err(TransferError::Nack) => Ok(false),
        Err(e) => Err(e),
    }
}

// Confirm an i2c-HID device the way the working monolithic driver did: address
// the HID descriptor register (0x0001) and read it back. ELAN and other i2c-HID
// parts NAK a bare read, so a register-addressed read is what actually detects
// the touchpad and selects the controller it hangs off. A descriptor length in
// the spec range distinguishes a real device from a stray address acknowledge.
pub fn probe_hid(driver: &Driver, addr: u8) -> Result<bool, TransferError> {
    // Retry a few times: a touchpad can be slow to answer the first transfer
    // right after its controller comes out of reset, and one miss must not make
    // the driver give up on the bus the device actually lives on.
    for _ in 0..4 {
        let req =
            TransferRequest { addr, flags: FLAG_RESTART_ON_READ, write: &[0x01, 0x00], read_len: 4 };
        match transfer(driver, req) {
            Ok(r) if r.read_len >= 2 => {
                let desc_len = u16::from_le_bytes([r.read[0], r.read[1]]);
                if (30..=256).contains(&desc_len) {
                    return Ok(true);
                }
            }
            Ok(_) | Err(TransferError::Nack) => {}
            Err(e) => return Err(e),
        }
        for _ in 0..200_000 {
            core::hint::spin_loop();
        }
    }
    Ok(false)
}
