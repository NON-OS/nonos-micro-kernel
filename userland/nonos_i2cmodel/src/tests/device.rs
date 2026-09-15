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

//! The device and the FIFOs: a reset acknowledged, a report set, a burst
//! that overruns.

use super::fixture::ready;
use crate::designware::regs::*;
use crate::{descriptor, Command, Config, Violation};

/// The command register, then `bytes`, then a STOP.
fn command(dw: &mut crate::Designware, bytes: &[u32]) {
    let reg = descriptor::REGISTERS.command.to_le_bytes();
    dw.write32(IC_DATA_CMD, reg[0] as u32);
    dw.write32(IC_DATA_CMD, reg[1] as u32);
    for (i, b) in bytes.iter().enumerate() {
        let stop = if i + 1 == bytes.len() { CMD_STOP } else { 0 };
        dw.write32(IC_DATA_CMD, b | stop);
    }
    dw.read32(IC_STATUS);
}

#[test]
fn a_reset_queues_the_zero_length_report_the_host_has_to_drain() {
    let (mut dw, pad) = ready(Config::LPSS, CON_RESTART_EN);
    command(&mut dw, &[0x00, 0x01]);
    assert_eq!(pad.lock().take_commands(), [Command::Reset]);
    assert_eq!(pad.lock().pending_inputs(), 1);
    dw.write32(IC_DATA_CMD, CMD_READ);
    dw.write32(IC_DATA_CMD, CMD_READ | CMD_STOP);
    assert_eq!((dw.read32(IC_DATA_CMD), dw.read32(IC_DATA_CMD)), (0, 0));
    assert_eq!(pad.lock().pending_inputs(), 0);
}

#[test]
fn a_set_report_lands_in_the_feature_store_id_first() {
    let (mut dw, pad) = ready(Config::LPSS, CON_RESTART_EN);
    let data = descriptor::REGISTERS.data.to_le_bytes();
    let bytes = [0x32, 0x03, data[0] as u32, data[1] as u32, 0x05, 0x00, 0x02, 0x03, 0x01];
    command(&mut dw, &bytes);
    assert_eq!(pad.lock().feature(2), Some(&[0x02, 0x03, 0x01][..]));
    assert!(matches!(pad.lock().commands()[0], Command::SetReport { ty: 3, id: 2, .. }));
}

#[test]
fn a_burst_past_the_fifo_depth_is_an_overflow_not_a_longer_queue() {
    let (mut dw, _) = ready(Config::SHALLOW, CON_RESTART_EN);
    for _ in 0..Config::SHALLOW.tx_depth + 1 {
        dw.write32(IC_DATA_CMD, 0x00);
    }
    assert!(dw.violations().contains(&Violation::TxOverflow));
    assert_eq!(dw.read32(IC_COMP_PARAM_1) >> 16 & 0xFF, Config::SHALLOW.tx_depth - 1);
}
