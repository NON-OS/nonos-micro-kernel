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

fn bit_len(mut v: u32) -> u32 {
    let mut n = 0;
    while v != 0 {
        v >>= 1;
        n += 1;
    }
    n
}

/// Remainder of `value` modulo `generator` over GF(2), reducing while the
/// dividend degree reaches the generator degree.
pub(super) fn poly_mod(mut value: u32, generator: u32) -> u32 {
    let g_len = bit_len(generator);
    while bit_len(value) >= g_len {
        value ^= generator << (bit_len(value) - g_len);
    }
    value
}
