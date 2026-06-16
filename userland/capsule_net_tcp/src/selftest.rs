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

#![cfg(feature = "tcp-selftest")]

use crate::tcp::seq;

pub fn run() -> u32 {
    let mut bits = 0u32;
    if seq_kat() {
        bits |= 1 << 0;
    }
    bits
}

fn seq_kat() -> bool {
    seq::lt(1, 2)
        && !seq::lt(2, 1)
        && seq::lt(0xFFFF_FFFF, 0)
        && seq::leq(5, 5)
        && seq::gt(6, 5)
        && seq::geq(5, 5)
        && seq::geq(6, 5)
        && seq::between(5, 1, 10)
        && !seq::between(10, 1, 10)
}
