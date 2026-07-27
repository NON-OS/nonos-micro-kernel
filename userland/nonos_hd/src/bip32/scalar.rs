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

// 256-bit scalar arithmetic modulo the secp256k1 group order, on four u64
// limbs. This is the only bignum BIP32 private derivation needs: the child
// key is (parent + tweak) mod n.

/// The secp256k1 group order n, big-endian.
pub const ORDER: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
];

fn to_limbs(bytes: &[u8; 32]) -> [u64; 4] {
    let mut limbs = [0u64; 4];
    for (i, limb) in limbs.iter_mut().enumerate() {
        let mut chunk = [0u8; 8];
        chunk.copy_from_slice(&bytes[i * 8..(i + 1) * 8]);
        *limb = u64::from_be_bytes(chunk);
    }
    limbs
}

/// True when `s` is a valid secp256k1 scalar for a private key or tweak:
/// nonzero and strictly below the group order.
pub fn is_valid_scalar(s: &[u8; 32]) -> bool {
    let v = to_limbs(s);
    let n = to_limbs(&ORDER);
    if v == [0, 0, 0, 0] {
        return false;
    }
    for i in 0..4 {
        if v[i] < n[i] {
            return true;
        }
        if v[i] > n[i] {
            return false;
        }
    }
    false // equal to the order is out of range
}

/// (a + b) mod n over the secp256k1 order. A 256-bit add can overflow n at
/// most once, so one conditional subtraction reduces completely; the equal
/// case reduces too, so the result is always strictly below n.
pub fn add_mod_n(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let av = to_limbs(a);
    let bv = to_limbs(b);
    let n = to_limbs(&ORDER);

    let mut sum = [0u64; 4];
    let mut carry = 0u64;
    for i in (0..4).rev() {
        let (s1, c1) = av[i].overflowing_add(bv[i]);
        let (s2, c2) = s1.overflowing_add(carry);
        sum[i] = s2;
        carry = (c1 as u64) + (c2 as u64);
    }

    // Reduce when the sum carried out of 256 bits or is >= n (the equal case
    // included: n itself is congruent to zero and must not survive).
    let mut ge = true;
    for i in 0..4 {
        if sum[i] > n[i] {
            break;
        }
        if sum[i] < n[i] {
            ge = false;
            break;
        }
    }
    if carry > 0 || ge {
        let mut borrow = 0u64;
        for i in (0..4).rev() {
            let (d1, b1) = sum[i].overflowing_sub(n[i]);
            let (d2, b2) = d1.overflowing_sub(borrow);
            sum[i] = d2;
            borrow = (b1 as u64) + (b2 as u64);
        }
    }

    let mut out = [0u8; 32];
    for (i, limb) in sum.iter().enumerate() {
        out[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_be_bytes());
    }
    out
}
