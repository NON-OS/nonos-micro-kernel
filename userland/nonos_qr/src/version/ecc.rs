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

//! Error-correction level and its format-info bits (ISO/IEC 18004 Table 12).

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Ecc {
    Low,
    Medium,
    Quartile,
    High,
}

impl Ecc {
    pub(crate) fn index(self) -> usize {
        match self {
            Ecc::Low => 0,
            Ecc::Medium => 1,
            Ecc::Quartile => 2,
            Ecc::High => 3,
        }
    }

    pub(crate) fn format_bits(self) -> u16 {
        match self {
            Ecc::Low => 0b01,
            Ecc::Medium => 0b00,
            Ecc::Quartile => 0b11,
            Ecc::High => 0b10,
        }
    }
}
