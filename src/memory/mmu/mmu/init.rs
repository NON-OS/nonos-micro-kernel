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

use super::super::error::MmuResult;
use super::core::MMU;
use super::protect;

impl MMU {
    /// Put the CPU's ring-0 restrictions in force, record what stuck, and
    /// adopt the page table the bootloader left live. Idempotent: a second
    /// call returns having touched no control register.
    pub fn initialize(&self) -> MmuResult<()> {
        let mut init_guard = self.initialized.lock();
        if *init_guard {
            return Ok(());
        }

        // Recorded from the read-back, so `get_protection_flags` answers with
        // what the hardware accepted rather than what was requested. A part
        // that silently dropped SMAP reads back false here and every caller
        // that gates on it sees the truth.
        *self.protection_flags.lock() = protect::apply()?;

        self.adopt_active_root()?;
        *init_guard = true;
        Ok(())
    }
}
