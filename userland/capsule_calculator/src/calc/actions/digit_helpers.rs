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

use crate::calc::fixed::{Fixed, FRAC};

pub fn integer_digit_count(magnitude: Fixed) -> u32 {
    let mut int_part = magnitude / FRAC;
    if int_part == 0 {
        return 1;
    }
    let mut count = 0;
    while int_part > 0 {
        int_part /= 10;
        count += 1;
    }
    count
}

pub fn pow10(exp: u32) -> Fixed {
    let mut value: Fixed = 1;
    for _ in 0..exp {
        value *= 10;
    }
    value
}
