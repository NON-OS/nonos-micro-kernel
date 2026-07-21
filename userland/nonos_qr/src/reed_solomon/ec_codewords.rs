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

use super::generator::generator;
use crate::gf256::mul;

/// The `ec` Reed-Solomon codewords for one data block: the remainder of the
/// data polynomial divided by the generator over GF(256).
pub(crate) fn ec_codewords(data: &[u8], ec: usize) -> Vec<u8> {
    let gen = generator(ec);
    let mut rem = alloc::vec![0u8; ec];
    for &d in data {
        let factor = d ^ rem[0];
        rem.remove(0);
        rem.push(0);
        if factor != 0 {
            for (r, &g) in rem.iter_mut().zip(gen.iter().skip(1)) {
                *r ^= mul(g, factor);
            }
        }
    }
    rem
}
