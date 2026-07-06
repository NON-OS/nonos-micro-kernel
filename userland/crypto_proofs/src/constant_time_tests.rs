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

use crate::crypto::constant_time::{
    ct_eq, ct_eq_16, ct_eq_32, ct_eq_64, ct_eq_u64, ct_gt_u64, ct_is_nonzero_u64, ct_is_zero_u64,
    ct_lt_u64, ct_select_u32, ct_select_u64, ct_select_u64_bit, ct_select_u8, ct_select_usize,
};

// The constant-time primitives gate every MAC, tag and signature comparison. A
// masking bug would silently accept or reject, so their functional correctness
// matters as much as their timing: each must equal the ordinary operation it
// replaces, for all inputs. (The timing property itself is by construction:
// branch-free bit operations that touch every byte.)

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn ct_eq_matches_slice_equality() {
    assert!(ct_eq(b"nonos", b"nonos"));
    assert!(!ct_eq(b"nonos", b"nonoS"));
    assert!(!ct_eq(b"nonos", b"nono")); // different length
    assert!(ct_eq(b"", b""));

    let mut s = 0x1234_5678_9abc_def0u64;
    for _ in 0..200_000 {
        let len = (xorshift(&mut s) % 40) as usize;
        let a: alloc::vec::Vec<u8> = (0..len).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        let mut b = a.clone();
        // Half the time perturb b (length or a byte).
        if xorshift(&mut s) & 1 == 0 && !b.is_empty() {
            let i = (xorshift(&mut s) as usize) % b.len();
            b[i] ^= 1 << (xorshift(&mut s) % 8);
        }
        assert_eq!(ct_eq(&a, &b), a == b);
    }
}

#[test]
fn ct_eq_fixed_widths_match_equality() {
    let mut s = 0xdead_beef_cafe_babeu64;
    for _ in 0..100_000 {
        let mut a16 = [0u8; 16];
        let mut a32 = [0u8; 32];
        let mut a64 = [0u8; 64];
        for b in a16.iter_mut() {
            *b = (xorshift(&mut s) & 0xff) as u8;
        }
        for b in a32.iter_mut() {
            *b = (xorshift(&mut s) & 0xff) as u8;
        }
        for b in a64.iter_mut() {
            *b = (xorshift(&mut s) & 0xff) as u8;
        }
        let (mut b16, mut b32, mut b64) = (a16, a32, a64);
        if xorshift(&mut s) & 1 == 0 {
            b16[(xorshift(&mut s) as usize) % 16] ^= 1;
        }
        if xorshift(&mut s) & 1 == 0 {
            b32[(xorshift(&mut s) as usize) % 32] ^= 1;
        }
        if xorshift(&mut s) & 1 == 0 {
            b64[(xorshift(&mut s) as usize) % 64] ^= 1;
        }
        assert_eq!(ct_eq_16(&a16, &b16), a16 == b16);
        assert_eq!(ct_eq_32(&a32, &b32), a32 == b32);
        assert_eq!(ct_eq_64(&a64, &b64), a64 == b64);
    }
}

#[test]
fn ct_integer_comparators_match_ordinary_comparison() {
    let mut s = 0x0f1e_2d3c_4b5a_6978u64;
    let edge = [0u64, 1, 2, u64::MAX, u64::MAX - 1, 1 << 63];
    for &a in &edge {
        for &b in &edge {
            assert_eq!(ct_eq_u64(a, b) != 0, a == b);
            assert_eq!(ct_lt_u64(a, b) != 0, a < b);
            assert_eq!(ct_gt_u64(a, b) != 0, a > b);
        }
        assert_eq!(ct_is_zero_u64(a) != 0, a == 0);
        assert_eq!(ct_is_nonzero_u64(a) != 0, a != 0);
    }
    for _ in 0..300_000 {
        let a = xorshift(&mut s);
        let b = xorshift(&mut s);
        assert_eq!(ct_eq_u64(a, b) != 0, a == b);
        assert_eq!(ct_lt_u64(a, b) != 0, a < b);
        assert_eq!(ct_gt_u64(a, b) != 0, a > b);
        assert_eq!(ct_is_zero_u64(a) != 0, a == 0);
        assert_eq!(ct_is_nonzero_u64(a) != 0, a != 0);
    }
}

#[test]
fn ct_select_matches_the_conditional_it_replaces() {
    let mut s = 0xa5a5_5a5a_c3c3_3c3cu64;
    for _ in 0..300_000 {
        let a = xorshift(&mut s);
        let b = xorshift(&mut s);
        assert_eq!(ct_select_u64(true, a, b), a);
        assert_eq!(ct_select_u64(false, a, b), b);
        assert_eq!(ct_select_usize(true, a as usize, b as usize), a as usize);
        assert_eq!(ct_select_usize(false, a as usize, b as usize), b as usize);
        assert_eq!(ct_select_u32(true, a as u32, b as u32), a as u32);
        assert_eq!(ct_select_u32(false, a as u32, b as u32), b as u32);
        assert_eq!(ct_select_u8(true, a as u8, b as u8), a as u8);
        assert_eq!(ct_select_u8(false, a as u8, b as u8), b as u8);
        assert_eq!(ct_select_u64_bit(1, a, b), a);
        assert_eq!(ct_select_u64_bit(0, a, b), b);
    }
}
