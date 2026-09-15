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

//! The master state machine: the command queue becomes bus traffic.

use super::record::Violation;
use super::regs::*;
use super::state::Designware;
use crate::bus::Dir;

impl Designware {
    /// Run every queued command. Called before each register read, so the
    /// core is always caught up with what the driver pushed.
    pub(super) fn step(&mut self) {
        while let Some(cmd) = self.tx.pop() {
            if !self.execute(cmd) {
                return;
            }
        }
    }

    /// One command. False when it aborted the transfer.
    fn execute(&mut self, cmd: u32) -> bool {
        let dir = if cmd & CMD_READ != 0 { Dir::Read } else { Dir::Write };
        let addr = (self.regs.get(&IC_TAR).copied().unwrap_or(0) & 0x7F) as u8;
        if !self.address_phase(addr, dir, cmd & CMD_RESTART != 0) {
            return self.abort(ABRT_7B_ADDR_NOACK);
        }
        if dir == Dir::Read {
            let byte = self.bus.read(cmd & CMD_STOP != 0);
            if !self.rx.push(byte) {
                self.violations.push(Violation::RxOverflow);
                self.raw_intr |= INTR_RX_OVER;
            }
        } else if !self.bus.write(cmd as u8) {
            return self.abort(ABRT_TXDATA_NOACK);
        }
        if cmd & CMD_STOP != 0 {
            self.bus.stop();
            self.active = false;
            self.dir = None;
        }
        true
    }
}
