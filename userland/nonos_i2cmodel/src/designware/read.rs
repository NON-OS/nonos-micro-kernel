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

//! Register reads. The core runs its command queue first: a real part works
//! the bus while the driver is looking at a register, so time passes here.

use super::record::Violation;
use super::regs::*;
use super::state::Designware;

impl Designware {
    pub fn read32(&mut self, offset: u64) -> u32 {
        self.step();
        if offset < CORE_END && !self.core_released() {
            self.violations.push(Violation::CoreTouchedInReset { offset });
            return 0;
        }
        match offset {
            IC_DATA_CMD => self.pop_rx(),
            IC_ENABLE => self.enable,
            IC_ENABLE_STATUS => self.enable_status(),
            IC_STATUS => self.status(),
            IC_TXFLR => self.tx.len(),
            IC_RXFLR => self.rx.len(),
            IC_RAW_INTR_STAT => self.raw_intr,
            IC_TX_ABRT_SOURCE => self.abort_source,
            IC_CLR_TX_ABRT => self.clear(INTR_TX_ABRT),
            IC_CLR_INTR => self.clear(u32::MAX),
            IC_COMP_PARAM_1 => self.config.comp_param_1(),
            IC_COMP_VERSION => COMP_VERSION_VALUE,
            IC_COMP_TYPE => COMP_TYPE_VALUE,
            LPSS_PRIV_RESETS => self.resets,
            other => self.regs.get(&other).copied().unwrap_or(0),
        }
    }

    fn pop_rx(&mut self) -> u32 {
        match self.rx.pop() {
            Some(byte) => byte as u32,
            None => {
                self.violations.push(Violation::ReadEmptyRx);
                0
            }
        }
    }
}
