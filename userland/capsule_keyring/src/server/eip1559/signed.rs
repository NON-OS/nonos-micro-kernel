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

use alloc::vec;
use alloc::vec::Vec;

use super::super::rlp::{rlp_list, rlp_uint_be};
use super::fields::{
    eth_transfer_fields, nox_approve_fields, nox_stake_approve_fields, nox_stake_fields,
    nox_stake_locked_fields, nox_transfer_fields, nox_unstake_fields,
};

pub fn signed_nox_transfer_tx(
    parts: (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 20], &[u8; 32]),
    y_parity: u8,
    r: &[u8; 32],
    s: &[u8; 32],
) -> Vec<u8> {
    let mut f = nox_transfer_fields(parts.0, parts.1, parts.2, parts.3, parts.4, parts.5);
    f.push(rlp_uint_be(&[y_parity]));
    f.push(rlp_uint_be(r));
    f.push(rlp_uint_be(s));
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}

pub fn signed_nox_stake_approve_tx(
    parts: (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32]),
    y_parity: u8,
    r: &[u8; 32],
    s: &[u8; 32],
) -> Vec<u8> {
    let mut f = nox_stake_approve_fields(parts.0, parts.1, parts.2, parts.3, parts.4);
    f.push(rlp_uint_be(&[y_parity]));
    f.push(rlp_uint_be(r));
    f.push(rlp_uint_be(s));
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}

pub fn signed_nox_stake_locked_tx(
    parts: (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32]),
    y_parity: u8,
    r: &[u8; 32],
    s: &[u8; 32],
) -> Vec<u8> {
    let mut f = nox_stake_locked_fields(parts.0, parts.1, parts.2, parts.3, parts.4, parts.5);
    f.push(rlp_uint_be(&[y_parity]));
    f.push(rlp_uint_be(r));
    f.push(rlp_uint_be(s));
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}

pub fn signed_nox_unstake_tx(
    parts: (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32]),
    y_parity: u8,
    r: &[u8; 32],
    s: &[u8; 32],
) -> Vec<u8> {
    let mut f = nox_unstake_fields(parts.0, parts.1, parts.2, parts.3, parts.4);
    f.push(rlp_uint_be(&[y_parity]));
    f.push(rlp_uint_be(r));
    f.push(rlp_uint_be(s));
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}

pub fn signed_nox_stake_tx(
    parts: (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32]),
    y_parity: u8,
    r: &[u8; 32],
    s: &[u8; 32],
) -> Vec<u8> {
    let mut f = nox_stake_fields(parts.0, parts.1, parts.2, parts.3, parts.4);
    f.push(rlp_uint_be(&[y_parity]));
    f.push(rlp_uint_be(r));
    f.push(rlp_uint_be(s));
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}

pub fn signed_nox_approve_tx(
    parts: (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32]),
    y_parity: u8,
    r: &[u8; 32],
    s: &[u8; 32],
) -> Vec<u8> {
    let mut f = nox_approve_fields(parts.0, parts.1, parts.2, parts.3, parts.4);
    f.push(rlp_uint_be(&[y_parity]));
    f.push(rlp_uint_be(r));
    f.push(rlp_uint_be(s));
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}

pub fn signed_eth_transfer_tx(
    parts: (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 20], &[u8; 32]),
    y_parity: u8,
    r: &[u8; 32],
    s: &[u8; 32],
) -> Vec<u8> {
    let mut f = eth_transfer_fields(parts.0, parts.1, parts.2, parts.3, parts.4, parts.5);
    f.push(rlp_uint_be(&[y_parity]));
    f.push(rlp_uint_be(r));
    f.push(rlp_uint_be(s));
    let mut out = vec![0x02u8];
    out.extend_from_slice(&rlp_list(&f));
    out
}
