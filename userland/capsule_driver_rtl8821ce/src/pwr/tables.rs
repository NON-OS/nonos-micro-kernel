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

//! The RTL8821CE MAC power transition tables, as command sequences for the
//! executor. These are the register values from the rtw88 `rtw8821c`
//! card-emulation-to-active transition, filtered to the PCIe interface and the
//! MAC register block and reimplemented as NONOS command tables (the values are
//! hardware facts; the code is original). Run with `run_pwr_seq`.

use super::command::PwrCmd;

/// Bring the MAC from card-emulation to active. This is the PCIe path: the
/// USB/SDIO-only steps and their millisecond delay do not apply, so the
/// sequence is pure register writes and two readiness polls.
pub const CARD_ENABLE: &[PwrCmd] = &[
    // Release the analog/digital power isolation and enable the power switches.
    PwrCmd::write(0x0005, 0x1C, 0x00),
    PwrCmd::write(0x0075, 0x01, 0x01),
    PwrCmd::poll(0x0006, 0x02, 0x02), // wait for the power-ready bit
    PwrCmd::write(0x0075, 0x01, 0x00),
    PwrCmd::write(0x0006, 0x01, 0x01),
    PwrCmd::write(0x0005, 0x80, 0x00),
    PwrCmd::write(0x0005, 0x18, 0x00),
    PwrCmd::write(0x0005, 0x01, 0x01),
    PwrCmd::poll(0x0005, 0x01, 0x00), // wait for the enable to settle
    PwrCmd::write(0x0020, 0x08, 0x08),
    // PCIe-specific: LDO, clock and reference settings.
    PwrCmd::write(0x0074, 0x20, 0x20),
    PwrCmd::write(0x0022, 0x02, 0x00),
    PwrCmd::write(0x0062, 0xE0, 0xE0),
    PwrCmd::write(0x0061, 0xE0, 0x00),
    PwrCmd::write(0x007C, 0x02, 0x00),
    PwrCmd::end(),
];
