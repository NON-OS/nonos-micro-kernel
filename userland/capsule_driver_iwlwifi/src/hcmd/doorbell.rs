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

//! Ring the transmit-queue doorbell. After a descriptor is placed at the
//! queue's write pointer, the firmware is told the new pointer by writing the
//! HBUS window write-pointer register with the queue id in the high byte and
//! the index in the low byte. Written against the `Mmio` trait so
//! `iwlwifi_proofs` drives it with a modeled device and checks the exact
//! register and value, with no hardware.

use crate::constants::HBUS_TARG_WRPTR;
use crate::regs::Mmio;

/// Publish `write_index` for transmit queue `queue` to the device.
pub fn ring<M: Mmio>(mmio: &M, queue: u8, write_index: u8) {
    mmio.write32(HBUS_TARG_WRPTR, ((queue as u32) << 8) | (write_index as u32));
}
