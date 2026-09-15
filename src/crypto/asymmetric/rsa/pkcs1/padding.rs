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

use crate::crypto::{CryptoError, CryptoResult};
use alloc::vec::Vec;

pub(super) fn pkcs1_pad_type1(data: &[u8], em_len: usize) -> CryptoResult<Vec<u8>> {
    if data.len() > em_len - 11 {
        return Err(CryptoError::InvalidLength);
    }
    let mut em = Vec::with_capacity(em_len);
    em.push(0x00);
    em.push(0x01);
    em.extend(core::iter::repeat_n(0xFF, em_len - data.len() - 3));
    em.push(0x00);
    em.extend_from_slice(data);
    Ok(em)
}

pub(super) fn pkcs1_unpad_type1(em: &[u8]) -> CryptoResult<Vec<u8>> {
    if em.len() < 11 {
        return Err(CryptoError::InvalidLength);
    }
    let mut valid: u8 = 1;
    valid &= ct_eq_u8(em[0], 0x00);
    valid &= ct_eq_u8(em[1], 0x01);
    let mut sep_idx: usize = 0;
    let mut found_sep: u8 = 0;
    let mut invalid_padding: u8 = 0;
    for (i, &byte) in em.iter().enumerate().skip(2) {
        let is_zero = ct_eq_u8(byte, 0x00);
        let is_ff = ct_eq_u8(byte, 0xFF);
        sep_idx = ct_select_usize(is_zero & (1 ^ found_sep), i, sep_idx);
        found_sep |= is_zero;
        invalid_padding |= (1 ^ found_sep) & (1 ^ is_ff) & (1 ^ is_zero);
    }
    valid &= found_sep;
    valid &= 1 ^ invalid_padding;
    valid &= ct_ge_usize(sep_idx, 10);
    if valid == 0 {
        return Err(CryptoError::InvalidLength);
    }
    Ok(em[sep_idx + 1..].to_vec())
}

#[inline]
fn ct_eq_u8(a: u8, b: u8) -> u8 {
    let diff = a ^ b;
    let is_zero = (diff as u16 | (diff as u16).wrapping_neg()) >> 8;
    (1 ^ is_zero) as u8
}
#[inline]
fn ct_select_usize(mask: u8, a: usize, b: usize) -> usize {
    let m = (mask as usize).wrapping_neg() & usize::MAX;
    (a & m) | (b & !m)
}
#[inline]
fn ct_ge_usize(a: usize, b: usize) -> u8 {
    let diff = a.wrapping_sub(b);
    let a_inv = !a;
    let borrow = ((a_inv & b) | ((a_inv | b) & diff)) >> (usize::BITS - 1);
    1 ^ (borrow as u8)
}
