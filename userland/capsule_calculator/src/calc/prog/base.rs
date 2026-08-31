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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Hex,
    Dec,
    Oct,
    Bin,
}

pub const BASES: [Base; 4] = [Base::Hex, Base::Dec, Base::Oct, Base::Bin];

impl Base {
    pub fn radix(self) -> u32 {
        match self {
            Base::Hex => 16,
            Base::Dec => 10,
            Base::Oct => 8,
            Base::Bin => 2,
        }
    }
    pub fn signed(self) -> bool {
        matches!(self, Base::Dec)
    }
    pub fn group(self) -> usize {
        match self {
            Base::Hex => 4,
            Base::Bin => 8,
            _ => 0,
        }
    }
    pub fn pad(self) -> usize {
        match self {
            Base::Hex => 8,
            Base::Bin => 32,
            _ => 0,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Base::Hex => "HEX",
            Base::Dec => "DEC",
            Base::Oct => "OCT",
            Base::Bin => "BIN",
        }
    }
}
