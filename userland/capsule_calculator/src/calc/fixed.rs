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

pub type Fixed = i128;

pub const FRAC: Fixed = 100_000_000;
pub const MAX_INTEGER_DIGITS: u32 = 16;
pub const MAX_FRACTION_DIGITS: u32 = 8;

pub fn from_digit(d: u8) -> Fixed {
    (d as Fixed) * FRAC
}

pub fn integer_part(value: Fixed) -> Fixed {
    value / FRAC
}

pub fn fraction_part(value: Fixed) -> Fixed {
    (value % FRAC).abs()
}
