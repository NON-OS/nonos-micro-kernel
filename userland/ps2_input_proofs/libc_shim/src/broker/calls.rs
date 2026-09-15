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

//! The grant calls the setup sequence makes. Grants are fixed numbers so a
//! test can tell them apart.

use super::table::acknowledge;
use super::types::{IrqBindOut, PioGrantOut, IRQ_GRANT_BASE, PIO_GRANT};

pub fn mk_device_claim(_device_id: u64) -> i64 {
    1
}

pub fn mk_pio_grant(_dev: u64, _epoch: u64, _bar: u8, _flags: u32, out: *mut PioGrantOut) -> i64 {
    /*
     * SAFETY: the driver passes a reference to a live PioGrantOut.
     */
    unsafe { (*out).grant_id = PIO_GRANT };
    0
}

pub fn mk_irq_bind(_dev: u64, _epoch: u64, line: u32, _f: u32, _n: u32, out: *mut IrqBindOut) -> i64 {
    /*
     * SAFETY: the driver passes a reference to a live IrqBindOut.
     */
    unsafe { (*out).grant_id = IRQ_GRANT_BASE + line as u64 };
    0
}

pub fn mk_irq_ack(grant_id: u64) -> i64 {
    acknowledge(grant_id);
    0
}

pub fn mk_device_release(_device_id: u64) -> i64 {
    0
}

pub fn mk_debug(_msg: *const u8, _len: usize) -> i64 {
    0
}
