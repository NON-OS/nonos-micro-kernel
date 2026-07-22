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

use alloc::vec::Vec;

use crate::gf256::{exp, mul};

// Generator polynomial of degree `ec`: the product of (x - a^i) for i in 0..ec,
// coefficients high-to-low.
pub(super) fn generator(ec: usize) -> Vec<u8> {
    let mut g = Vec::with_capacity(ec + 1);
    g.push(1u8);
    for i in 0..ec {
        let mut next = alloc::vec![0u8; g.len() + 1];
        let root = exp(i);
        for (j, &c) in g.iter().enumerate() {
            next[j] ^= c;
            next[j + 1] ^= mul(c, root);
        }
        g = next;
    }
    g
}
