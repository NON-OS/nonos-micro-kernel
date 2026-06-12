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

use super::super::rlp::{rlp_list, rlp_string, rlp_uint_be};
use super::approve_data::approve_calldata;
use super::consts::{CHAIN_ID, NOX_TOKEN};

pub fn tx_fields(
    chain_id: &[u8],
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    to: &[u8; 20],
    value: &[u8; 32],
    data: &[u8],
) -> Vec<Vec<u8>> {
    let mut f = Vec::with_capacity(9);
    f.push(rlp_uint_be(chain_id));
    f.push(rlp_uint_be(nonce));
    f.push(rlp_uint_be(max_priority));
    f.push(rlp_uint_be(max_fee));
    f.push(rlp_uint_be(gas));
    f.push(rlp_string(to));
    f.push(rlp_uint_be(value));
    f.push(rlp_string(data));
    f.push(rlp_list(&[]));
    f
}

pub fn nox_approve_fields(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    amount: &[u8; 32],
) -> Vec<Vec<u8>> {
    tx_fields(
        &[CHAIN_ID],
        nonce,
        max_priority,
        max_fee,
        gas,
        &NOX_TOKEN,
        &[0u8; 32],
        &approve_calldata(amount),
    )
}

pub fn eth_transfer_fields(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    to: &[u8; 20],
    value: &[u8; 32],
) -> Vec<Vec<u8>> {
    tx_fields(&[CHAIN_ID], nonce, max_priority, max_fee, gas, to, value, &[])
}
