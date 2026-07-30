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

//! The memory types this kernel maps, and the MAIR_EL1 slot each one owns.
//!
//! A page descriptor names its memory type by index, and MAIR_EL1 says what
//! that index means. The two halves must agree or every mapping silently gets
//! the wrong caching and ordering rules, so both live here and `control::mair`
//! builds the register by walking [`MemoryType::ALL`] rather than repeating the
//! table by hand.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    DeviceNGnRnE,
    DeviceNGnRE,
    DeviceNGRE,
    DeviceGRE,
    NormalNC,
    NormalWT,
    NormalWB,
}

impl MemoryType {
    /// Every type, in MAIR slot order. `control::mair` iterates this.
    pub const ALL: [Self; 7] = [
        Self::DeviceNGnRnE,
        Self::DeviceNGnRE,
        Self::DeviceNGRE,
        Self::DeviceGRE,
        Self::NormalNC,
        Self::NormalWT,
        Self::NormalWB,
    ];

    /// The MAIR_EL1 slot this type occupies, as written into `AttrIndx[2:0]`.
    pub const fn attr_index(&self) -> u64 {
        match self {
            Self::DeviceNGnRnE => 0,
            Self::DeviceNGnRE => 1,
            Self::DeviceNGRE => 2,
            Self::DeviceGRE => 3,
            Self::NormalNC => 4,
            Self::NormalWT => 5,
            Self::NormalWB => 6,
        }
    }

    /// The MAIR_EL1 attribute byte for that slot.
    ///
    /// A zero high nibble marks Device memory, and the low nibble then picks
    /// the gathering, reordering and early-ack rules. Both nibbles non-zero
    /// marks Normal memory: outer cacheability high, inner low.
    pub const fn mair_attr(&self) -> u8 {
        match self {
            Self::DeviceNGnRnE => 0x00,
            Self::DeviceNGnRE => 0x04,
            Self::DeviceNGRE => 0x08,
            Self::DeviceGRE => 0x0C,
            Self::NormalNC => 0x44,
            Self::NormalWT => 0xBB,
            Self::NormalWB => 0xFF,
        }
    }
}
