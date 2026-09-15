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

//! A write transfer, read off the wire.

use nonos_i2cmodel::{BusEvent, Command, Config, Dir};

use super::fixture::{bench, core, PAD};
use crate::transaction::{transfer, TransferRequest};

/// SET_POWER ON to the command register, as the HID driver sends it.
const SET_POWER_ON: [u8; 4] = [0x05, 0x00, 0x00, 0x08];

#[test]
fn a_write_is_one_start_the_bytes_in_order_and_one_stop() {
    let b = bench(Config::LPSS);
    let req = TransferRequest { addr: PAD, flags: 0, write: &SET_POWER_ON, read_len: 0 };
    transfer(&b.driver, req).ok().expect("write transfer");

    let trace = core(|c| c.bus().trace().to_vec());
    assert_eq!(trace[0], BusEvent::Start { addr: PAD, dir: Dir::Write, acked: true });
    let written: Vec<u8> = trace.iter().filter_map(BusEvent::written).collect();
    assert_eq!(written, SET_POWER_ON, "bytes on the wire differ from the request");
    assert_eq!(trace.iter().filter(|e| e.is_stop()).count(), 1, "one STOP closes a write");
    assert!(trace.last().is_some_and(BusEvent::is_stop), "the STOP comes last");
    assert_eq!(b.pad.lock().commands(), [Command::SetPower { sleep: false }]);
}

#[test]
fn the_controller_is_left_disabled_and_broke_no_rule_getting_there() {
    /*
     * The engine enables the core for the transfer and disables it after.
     * A driver that left it enabled would have every later register write
     * dropped, since the configuration space is read-only while enabled.
     */
    let b = bench(Config::LPSS);
    let req = TransferRequest { addr: PAD, flags: 0, write: &SET_POWER_ON, read_len: 0 };
    transfer(&b.driver, req).ok().expect("write transfer");
    core(|c| {
        assert!(!c.is_enabled(), "controller left enabled after the transfer");
        assert!(c.violations().is_empty(), "{:?}", c.violations());
    });
}

#[test]
fn a_write_longer_than_the_wire_format_allows_is_refused_before_the_bus() {
    let b = bench(Config::LPSS);
    let too_long = [0u8; crate::protocol::TRANSFER_WRITE_MAX + 1];
    let req = TransferRequest { addr: PAD, flags: 0, write: &too_long, read_len: 0 };
    assert!(matches!(transfer(&b.driver, req), Err(crate::transaction::TransferError::Invalid)));
    assert!(core(|c| c.bus().trace().is_empty()), "the bus was touched for a refused request");
}
