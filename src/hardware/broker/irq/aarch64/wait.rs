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

//! Blocking IRQ wait is not implemented on the GIC backend yet;
//! capsules fall back to `MkIrqPoll`.

use crate::hardware::broker::irq::types::IrqError;

pub fn wait_arm(_pid: u32, _grant_id: u64) -> Result<u64, IrqError> {
    Err(IrqError::PlatformError)
}

pub fn wait_disarm(_pid: u32, _grant_id: u64) -> Result<u64, IrqError> {
    Err(IrqError::PlatformError)
}
