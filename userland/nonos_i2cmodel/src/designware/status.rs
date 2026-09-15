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

//! The registers the driver spins on, derived from the core's state.

use super::regs::*;
use super::state::Designware;

impl Designware {
    /// IC_ENABLE_STATUS. The frozen variant reports the core as enabled
    /// whatever was written, which is how a clock-gated part reads.
    pub(super) fn enable_status(&self) -> u32 {
        if self.enable_follows {
            self.enable & 1
        } else {
            1
        }
    }

    pub(super) fn status(&self) -> u32 {
        let busy = self.active || !self.tx.is_empty();
        let mut s = 0;
        if busy {
            s |= STATUS_ACTIVITY | STATUS_MST_ACTIVITY;
        }
        if !self.tx.is_full() {
            s |= STATUS_TFNF;
        }
        if self.tx.is_empty() {
            s |= STATUS_TFE;
        }
        if !self.rx.is_empty() {
            s |= STATUS_RFNE;
        }
        if self.rx.is_full() {
            s |= STATUS_RFF;
        }
        s
    }

    /// Reading a clear register acknowledges the interrupt. The abort source
    /// goes with the abort, as the databook says it does.
    pub(super) fn clear(&mut self, bits: u32) -> u32 {
        self.raw_intr &= !bits;
        if bits & INTR_TX_ABRT != 0 {
            self.abort_source = 0;
        }
        0
    }
}
