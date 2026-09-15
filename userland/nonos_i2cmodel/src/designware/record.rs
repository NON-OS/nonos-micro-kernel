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

//! What the core noticed the driver doing: the register writes worth
//! ordering, and the acts the databook forbids.

/// Every register write, in order, with the value. A proof reads the
/// bring-up sequence off this.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RegEvent {
    pub offset: u64,
    pub value: u32,
}

/// Something the databook says a driver must not do. Real silicon does not
/// report these; it drops the write, runs the bus at the wrong speed, or
/// loses a byte, and the failure shows up somewhere else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Violation {
    /// A core register touched while the LPSS wrapper still held the core in
    /// reset. Every such access reads garbage on the real part.
    CoreTouchedInReset { offset: u64 },
    /// A configuration register written while IC_ENABLE was set. The core
    /// ignores the write.
    WriteWhileEnabled { offset: u64 },
    /// A command pushed while the core was disabled. It goes nowhere.
    CommandWhileDisabled,
    /// More commands pushed than the TX FIFO holds. The extra ones are lost.
    TxOverflow,
    /// More bytes read from the bus than the RX FIFO holds. The extra ones
    /// are lost and the transfer returns short.
    RxOverflow,
    /// IC_DATA_CMD read with nothing in the RX FIFO. The value is stale.
    ReadEmptyRx,
    /// A read command after a write, or the reverse, with neither the
    /// RESTART bit set nor IC_CON.RESTART_EN. The core issues a STOP and a
    /// fresh START, and a register-addressed device loses the register.
    DirectionChangeWithoutRestart,
    /// A target address wider than seven bits, which on this core selects
    /// ten-bit addressing rather than a different device.
    TenBitTarget { value: u32 },
}
