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

use crate::calc::fixed::{Fixed, FRAC, MAX_FRACTION_DIGITS};
use crate::calc::state::State;

use super::digit_helpers::{integer_digit_count, pow10};

const INTEGER_DIGIT_LIMIT: u32 = 16;

pub fn run(state: &mut State, digit: u8) {
    if state.is_error() || digit > 9 {
        return;
    }
    if state.new_input {
        state.display = (digit as Fixed) * FRAC;
        state.new_input = false;
        state.decimal_digits_typed = 0;
        return;
    }
    let sign: Fixed = if state.display < 0 { -1 } else { 1 };
    let magnitude = state.display.saturating_abs();
    let next = if state.decimal_digits_typed == 0 {
        if integer_digit_count(magnitude) >= INTEGER_DIGIT_LIMIT {
            return;
        }
        let int_part = magnitude / FRAC;
        let frac_part = magnitude % FRAC;
        int_part
            .saturating_mul(10)
            .saturating_add(digit as Fixed)
            .saturating_mul(FRAC)
            .saturating_add(frac_part)
    } else {
        if state.decimal_digits_typed >= MAX_FRACTION_DIGITS as u8 {
            return;
        }
        let shift = MAX_FRACTION_DIGITS - 1 - state.decimal_digits_typed as u32;
        let increment = (digit as Fixed) * pow10(shift);
        magnitude.saturating_add(increment)
    };
    state.display = sign * next;
    if state.decimal_digits_typed >= 1 && state.decimal_digits_typed < MAX_FRACTION_DIGITS as u8 {
        state.decimal_digits_typed += 1;
    }
}
