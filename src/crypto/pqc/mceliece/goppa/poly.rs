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

extern crate alloc;

use super::super::gf::GF2m;
use super::super::FIELD_SIZE;
use alloc::vec::Vec;

pub(crate) fn poly_eval(coeffs: &[u16], x: u16) -> u16 {
    let mut result = 0u16;
    let mut power = 1u16;

    for &coeff in coeffs {
        result = GF2m::add(result, GF2m::mul(coeff, power));
        power = GF2m::mul(power, x);
    }

    result
}

pub(crate) fn poly_degree(p: &[u16]) -> usize {
    for i in (0..p.len()).rev() {
        if p[i] != 0 {
            return i;
        }
    }
    usize::MAX
}

pub(crate) fn poly_gcd(a: &[u16], b: &[u16]) -> Vec<u16> {
    let mut u = a.to_vec();
    let mut v = b.to_vec();

    while !v.iter().all(|&x| x == 0) {
        let u_deg = poly_degree(&u);
        let v_deg = poly_degree(&v);

        if u_deg < v_deg {
            core::mem::swap(&mut u, &mut v);
            continue;
        }

        if v_deg == usize::MAX {
            break;
        }

        let lead_v = v[v_deg];
        let lead_v_inv = GF2m::inv(lead_v);

        let shift = u_deg - v_deg;
        let coeff = GF2m::mul(u[u_deg], lead_v_inv);

        for i in 0..=v_deg {
            if i + shift < u.len() {
                u[i + shift] = GF2m::add(u[i + shift], GF2m::mul(v[i], coeff));
            }
        }
    }

    u
}

pub(crate) fn is_likely_irreducible(poly: &[u16]) -> bool {
    for x in 1..FIELD_SIZE as u16 {
        if poly_eval(poly, x) == 0 {
            return false;
        }
    }
    true
}
