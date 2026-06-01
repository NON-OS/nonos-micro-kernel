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

pub fn base_fields(
    nonce: &[u8; 32],
    max_priority: &[u8; 32],
    max_fee: &[u8; 32],
    gas: &[u8; 32],
    amount: &[u8; 32],
) -> Vec<Vec<u8>> {
    let mut f = Vec::with_capacity(9);
    f.push(rlp_uint_be(&[CHAIN_ID]));
    f.push(rlp_uint_be(nonce));
    f.push(rlp_uint_be(max_priority));
    f.push(rlp_uint_be(max_fee));
    f.push(rlp_uint_be(gas));
    f.push(rlp_string(&NOX_TOKEN));
    f.push(rlp_uint_be(&[]));
    f.push(rlp_string(&approve_calldata(amount)));
    f.push(rlp_list(&[]));
    f
}
