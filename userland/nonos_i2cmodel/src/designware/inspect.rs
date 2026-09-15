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

//! What a proof asks the core after the driver has run.

use super::record::{RegEvent, Violation};
use super::regs::LPSS_RESETS_RELEASED;
use super::state::Designware;
use crate::bus::Bus;

impl Designware {
    /// The same core with IC_ENABLE_STATUS frozen where it is.
    pub fn never_acknowledges_enable(mut self) -> Self {
        self.enable_follows = false;
        self
    }

    pub fn bus(&self) -> &Bus {
        &self.bus
    }

    pub fn violations(&self) -> &[Violation] {
        &self.violations
    }

    /// Every register write in order, so a sequence can be checked as a
    /// sequence and not as a final state.
    pub fn events(&self) -> &[RegEvent] {
        &self.events
    }

    /// The values written at `offset`, oldest first.
    pub fn writes_to(&self, offset: u64) -> Vec<u32> {
        self.events.iter().filter(|e| e.offset == offset).map(|e| e.value).collect()
    }

    pub fn is_enabled(&self) -> bool {
        self.enable & 1 != 0
    }

    pub fn core_released(&self) -> bool {
        self.resets & LPSS_RESETS_RELEASED == LPSS_RESETS_RELEASED
    }

    pub fn rx_pending(&self) -> u32 {
        self.rx.len()
    }
}
