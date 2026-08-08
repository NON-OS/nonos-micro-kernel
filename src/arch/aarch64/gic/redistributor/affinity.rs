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

use super::constants::GICR_TYPER;
use super::device::GicRedistributor;

impl GicRedistributor {
    /// The whole of `GICR_TYPER`: who owns this frame, whether it is the last
    /// one in the region, and whether it carries the virtual-LPI pages.
    pub(super) fn typer(&self) -> u64 {
        self.read_reg64(GICR_TYPER)
    }

    /// The affinity of the CPU this frame belongs to, packed `Aff3:Aff2:Aff1:Aff0`.
    pub fn affinity(&self) -> u32 {
        (self.typer() >> 32) as u32
    }
}
