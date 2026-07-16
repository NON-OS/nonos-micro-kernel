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

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AllocFlags: u32 {
        const EMPTY = 0;
        const ZERO = 1 << 0;
        const HIGH = 1 << 1;
        const DMA = 1 << 2;
        const CONTIGUOUS = 1 << 3;
        /// The allocation must be below 4GB so a 32-bit device DMA descriptor can
        /// address it without truncation.
        const DMA32 = 1 << 4;
    }
}

impl Default for AllocFlags {
    fn default() -> Self {
        Self::EMPTY
    }
}
