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

//! The controller: reset gating, a register read typed the way the databook
//! describes one, and an abort.

use super::fixture::{parts, ready};
use crate::designware::regs::*;
use crate::{descriptor, Config, Violation};

#[test]
fn the_core_is_dead_until_the_lpss_reset_is_released() {
    let (mut dw, _) = parts(Config::LPSS);
    assert_eq!(dw.read32(IC_COMP_TYPE), 0);
    assert_eq!(dw.violations(), [Violation::CoreTouchedInReset { offset: IC_COMP_TYPE }]);
    dw.write32(LPSS_PRIV_RESETS, LPSS_RESETS_RELEASED);
    assert_eq!(dw.read32(IC_COMP_TYPE), COMP_TYPE_VALUE);
}

#[test]
fn a_register_read_typed_by_hand_returns_the_hid_descriptor() {
    let (mut dw, pad) = ready(Config::LPSS, CON_RESTART_EN);
    for cmd in [0x01, 0x00, CMD_READ | CMD_RESTART, CMD_READ | CMD_STOP] {
        dw.write32(IC_DATA_CMD, cmd);
    }
    assert_eq!(dw.read32(IC_RXFLR), 2);
    let len = u16::from_le_bytes([dw.read32(IC_DATA_CMD) as u8, dw.read32(IC_DATA_CMD) as u8]);
    assert_eq!(len as usize, descriptor::LEN);
    assert_eq!(dw.read32(IC_STATUS) & STATUS_MST_ACTIVITY, 0, "transfer still open after STOP");
    assert!(dw.violations().is_empty(), "{:?}", dw.violations());
    assert_eq!(pad.lock().commands().len(), 1);
}

#[test]
fn turning_round_without_restart_enabled_closes_the_transfer_first() {
    let (mut dw, pad) = ready(Config::LPSS, 0);
    for cmd in [0x01, 0x00, CMD_READ | CMD_STOP] {
        dw.write32(IC_DATA_CMD, cmd);
    }
    dw.read32(IC_STATUS);
    assert_eq!(dw.violations(), [Violation::DirectionChangeWithoutRestart]);
    /*
     * The device saw a STOP, forgot the register, and served the input
     * register to the fresh START instead of the descriptor.
     */
    assert!(pad.lock().commands().iter().any(|c| matches!(c, crate::Command::InputRead)));
}

#[test]
fn an_empty_address_aborts_with_the_address_nack_source() {
    let (mut dw, _) = parts(Config::LPSS);
    dw.write32(LPSS_PRIV_RESETS, LPSS_RESETS_RELEASED);
    dw.write32(IC_TAR, 0x2C);
    dw.write32(IC_ENABLE, 1);
    dw.write32(IC_DATA_CMD, CMD_READ | CMD_STOP);
    assert_eq!(dw.read32(IC_RAW_INTR_STAT) & INTR_TX_ABRT, INTR_TX_ABRT);
    assert_eq!(dw.read32(IC_TX_ABRT_SOURCE), ABRT_7B_ADDR_NOACK);
    dw.read32(IC_CLR_TX_ABRT);
    assert_eq!(dw.read32(IC_TX_ABRT_SOURCE), 0, "clearing the abort keeps its source");
}
