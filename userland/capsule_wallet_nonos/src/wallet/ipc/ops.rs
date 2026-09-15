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

//! The client's half of the operation table, checked against itself.
//!
//! The keyring holds the same list and refuses to build on a repeat. This one
//! matters just as much: a client that sends 19 for an unstake when the server
//! reads 19 as a wallet generation gets an error at best, and at worst a
//! handler acting on bytes meant for another. Two of these did collide.

use super::constants::*;

pub const ALL: [u16; 15] = [
    OP_WALLET_IMPORT,
    OP_WALLET_ADDRESS,
    OP_SIGN_NOX_APPROVE,
    OP_SIGN_ETH_TRANSFER,
    OP_LIST_WALLET_RAILS,
    OP_WALLET_EXPORT,
    OP_SIGN_NOX_STAKE_APPROVE,
    OP_SIGN_NOX_STAKE,
    OP_SIGN_NOX_TRANSFER,
    OP_SIGN_NOX_UNSTAKE,
    OP_SIGN_NOX_STAKE_LOCKED,
    OP_WALLET_GENERATE_HD,
    OP_WALLET_RECOVER,
    OP_VAULT_SEAL,
    OP_VAULT_OPEN,
];

pub const fn all_distinct(ops: &[u16]) -> bool {
    let mut i = 0;
    while i < ops.len() {
        let mut j = i + 1;
        while j < ops.len() {
            if ops[i] == ops[j] {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const _: () = assert!(all_distinct(&ALL), "two operations share a code");
