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

//! Register writes: configuration takes only while disabled, the data
//! register queues a command, and dropping the enable flushes that queue.

use super::record::{RegEvent, Violation};
use super::regs::*;
use super::state::Designware;

impl Designware {
    pub fn write32(&mut self, offset: u64, value: u32) {
        self.events.push(RegEvent { offset, value });
        if offset == LPSS_PRIV_RESETS {
            self.resets = value;
            return;
        }
        if offset < CORE_END && !self.core_released() {
            self.violations.push(Violation::CoreTouchedInReset { offset });
            return;
        }
        match offset {
            IC_ENABLE => self.set_enable(value),
            IC_DATA_CMD => self.push_command(value),
            // The interrupt mask is the one register writable at any time.
            IC_INTR_MASK => _ = self.regs.insert(offset, value),
            IC_TAR if value > 0x7F => {
                self.violations.push(Violation::TenBitTarget { value });
                self.configure(offset, value);
            }
            _ => self.configure(offset, value),
        }
    }

    fn push_command(&mut self, value: u32) {
        if !self.is_enabled() {
            self.violations.push(Violation::CommandWhileDisabled);
        } else if !self.tx.push(value) {
            self.violations.push(Violation::TxOverflow);
            self.raw_intr |= INTR_TX_OVER;
        }
    }

    fn configure(&mut self, offset: u64, value: u32) {
        if self.is_enabled() {
            self.violations.push(Violation::WriteWhileEnabled { offset });
        } else {
            self.regs.insert(offset, value);
        }
    }
}
