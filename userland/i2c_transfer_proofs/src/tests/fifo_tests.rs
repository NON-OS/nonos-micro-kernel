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

//! FIFO discipline. The core says how deep its FIFOs are in IC_COMP_PARAM_1,
//! and silicon ships with 8, 16, 32 and 64. A command pushed into a full TX
//! FIFO is lost; a byte read into a full RX FIFO is lost. Neither is
//! reported to the caller as anything but a transfer that came back wrong.

use nonos_i2cmodel::{descriptor, Config, Violation};

use super::fixture::{bench, core, PAD};
use crate::transaction::{transfer, TransferRequest, FLAG_RESTART_ON_READ};

const HID_DESC_REG: [u8; 2] = [0x01, 0x00];

fn descriptor_read(config: Config) -> Vec<Violation> {
    let b = bench(config);
    let req = TransferRequest {
        addr: PAD,
        flags: FLAG_RESTART_ON_READ,
        write: &HID_DESC_REG,
        read_len: descriptor::LEN,
    };
    let result = transfer(&b.driver, req).ok().expect("register read");
    assert_eq!(&result.read[..result.read_len], b.pad.lock().descriptor(), "bytes lost");
    core(|c| c.violations().to_vec())
}

#[test]
fn the_engine_never_pushes_more_commands_than_the_core_reports_room_for() {
    /*
     * A driver that assumed the LPSS depth of 64 pushes 32 commands into an
     * eight-entry FIFO and loses 24 of them; the transfer then completes
     * short and the caller sees a device returning garbage.
     */
    let violations = descriptor_read(Config::SHALLOW);
    assert!(!violations.contains(&Violation::TxOverflow), "{violations:?}");
}

#[test]
fn outstanding_reads_never_exceed_the_room_left_in_the_receive_fifo() {
    /*
     * Reads are issued as commands and land as bytes later. IC_RXFLR counts
     * bytes that have landed, not reads in flight, so a driver that gates on
     * it alone can have more bytes on their way than the FIFO will hold.
     */
    let violations = descriptor_read(Config { tx_depth: 64, rx_depth: 8 });
    assert!(!violations.contains(&Violation::RxOverflow), "{violations:?}");
}

#[test]
fn on_the_lpss_depths_a_full_register_read_is_clean() {
    let violations = descriptor_read(Config::LPSS);
    assert!(violations.is_empty(), "{violations:?}");
}
