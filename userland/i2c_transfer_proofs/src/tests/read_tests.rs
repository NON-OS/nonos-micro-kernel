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

//! A register read: two bytes of address, a repeated START, the bytes back.
//! This is the transfer every HID-over-I2C exchange is built from, and the
//! repeated START is what makes it one transaction to the device.

use nonos_i2cmodel::{descriptor, BusEvent, Config, Dir};

use super::fixture::{bench, core, PAD};
use crate::transaction::{transfer, TransferRequest, FLAG_RESTART_ON_READ};

pub(super) const HID_DESC_REG: [u8; 2] = [0x01, 0x00];

#[test]
fn a_register_read_returns_the_register_and_turns_round_with_a_repeated_start() {
    let b = bench(Config::LPSS);
    let req = TransferRequest {
        addr: PAD,
        flags: FLAG_RESTART_ON_READ,
        write: &HID_DESC_REG,
        read_len: descriptor::LEN,
    };
    let result = transfer(&b.driver, req).ok().expect("register read");
    assert_eq!(result.read_len, descriptor::LEN);
    assert_eq!(&result.read[..descriptor::LEN], b.pad.lock().descriptor());

    let trace = core(|c| c.bus().trace().to_vec());
    let restarts: Vec<_> = trace.iter().filter(|e| e.is_restart()).collect();
    assert_eq!(restarts, [&BusEvent::Restart { addr: PAD, dir: Dir::Read, acked: true }]);
    assert_eq!(trace.iter().filter(|e| e.is_stop()).count(), 1, "a read is one transaction");
    assert!(core(|c| c.violations().is_empty()), "{:?}", core(|c| c.violations().to_vec()));
}

#[test]
fn the_direction_change_gets_a_repeated_start_even_without_the_flag() {
    /*
     * Bring-up sets IC_CON.RESTART_EN, so the core repeats the START on its
     * own when a read follows a write. The flag only forces it. A bring-up
     * that dropped RESTART_EN would make every unflagged register read a
     * STOP and START pair, and this device forgets its register on a STOP.
     */
    let b = bench(Config::LPSS);
    let req = TransferRequest { addr: PAD, flags: 0, write: &HID_DESC_REG, read_len: 2 };
    let result = transfer(&b.driver, req).ok().expect("register read");
    assert_eq!(u16::from_le_bytes([result.read[0], result.read[1]]), descriptor::LEN as u16);
    assert!(core(|c| c.violations().is_empty()), "{:?}", core(|c| c.violations().to_vec()));
}
