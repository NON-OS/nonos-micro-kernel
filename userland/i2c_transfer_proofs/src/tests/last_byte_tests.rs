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

//! How a read ends on the wire. The device learns a read is over from the
//! NACK on its final byte: a controller that NACKs early truncates, one that
//! never NACKs leaves the device driving SDA into the STOP.

use nonos_i2cmodel::{BusEvent, Config};

use super::fixture::{bench, core, PAD};
use super::read_tests::HID_DESC_REG;
use crate::transaction::{transfer, TransferRequest, FLAG_RESTART_ON_READ};

#[test]
fn the_last_byte_read_is_the_one_the_controller_nacks() {
    let b = bench(Config::LPSS);
    let req = TransferRequest {
        addr: PAD,
        flags: FLAG_RESTART_ON_READ,
        write: &HID_DESC_REG,
        read_len: 4,
    };
    transfer(&b.driver, req).ok().expect("register read");
    let lasts: Vec<bool> = core(|c| {
        c.bus()
            .trace()
            .iter()
            .filter_map(|e| match e {
                BusEvent::Read { last, .. } => Some(*last),
                _ => None,
            })
            .collect()
    });
    assert_eq!(lasts, [false, false, false, true]);
}

#[test]
fn a_one_byte_read_is_nacked_on_its_only_byte() {
    let b = bench(Config::LPSS);
    let req = TransferRequest {
        addr: PAD,
        flags: FLAG_RESTART_ON_READ,
        write: &HID_DESC_REG,
        read_len: 1,
    };
    transfer(&b.driver, req).ok().expect("register read");
    let reads: Vec<_> = core(|c| {
        c.bus().trace().iter().filter(|e| matches!(e, BusEvent::Read { .. })).copied().collect()
    });
    assert_eq!(reads, [BusEvent::Read { byte: 30, last: true }]);
}
