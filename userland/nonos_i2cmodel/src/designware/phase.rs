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

//! Opening and closing a transfer on the bus.

use super::record::Violation;
use super::regs::*;
use super::state::Designware;
use crate::bus::Dir;

impl Designware {
    /// A START when the bus is idle, a repeated START on a direction change
    /// or when asked for one, nothing when the direction holds. Without
    /// RESTART_EN the core cannot repeat a START and closes the transfer
    /// with a STOP instead, which is what a register-addressed device sees
    /// as a fresh transaction.
    pub(super) fn address_phase(&mut self, addr: u8, dir: Dir, restart: bool) -> bool {
        let same_dir = self.dir == Some(dir);
        let acked = if !self.active {
            self.bus.start(addr, dir, false)
        } else if same_dir && !restart {
            true
        } else if self.regs.get(&IC_CON).copied().unwrap_or(0) & CON_RESTART_EN != 0 {
            self.bus.start(addr, dir, true)
        } else {
            self.violations.push(Violation::DirectionChangeWithoutRestart);
            self.bus.stop();
            self.bus.start(addr, dir, false)
        };
        self.active = acked;
        self.dir = acked.then_some(dir);
        acked
    }

    /// Disabling aborts whatever was in flight and empties the TX FIFO, as
    /// the databook describes for IC_ENABLE.ENABLE going low.
    pub(super) fn set_enable(&mut self, value: u32) {
        self.enable = value & 1;
        if self.enable == 0 {
            self.tx.clear();
            if self.active {
                self.bus.stop();
            }
            self.active = false;
            self.dir = None;
        }
    }

    /// The transfer dies: the abort is latched with its source, the queue
    /// is flushed and the bus released. Always false, for the caller's loop.
    pub(super) fn abort(&mut self, source: u32) -> bool {
        self.abort_source |= source;
        self.raw_intr |= INTR_TX_ABRT;
        self.tx.clear();
        if self.active {
            self.bus.stop();
        }
        self.active = false;
        self.dir = None;
        false
    }
}
