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

//! Every operation code, in one list, checked to be distinct at compile time.
//!
//! Two of them were not. `OP_WALLET_GENERATE_HD` and `OP_SIGN_NOX_UNSTAKE`
//! were both 19, and `OP_WALLET_RECOVER` and `OP_SIGN_NOX_STAKE_LOCKED` were
//! both 20, so the two sign paths sat behind an earlier match arm and could
//! not be reached: an unstake request answered EINVAL from a handler that was
//! reading its bytes as a wallet generation. The compiler said so on every
//! build, as an unreachable pattern, and the warning was passed over.
//!
//! Naming the numbers once and refusing to compile a repeat is cheaper than
//! reading warnings. A new op goes in [`ALL`] as well as beside its name.

use super::types::*;

pub const ALL: [u16; 24] = [
    OP_STORE,
    OP_RETRIEVE,
    OP_DELETE,
    OP_LOCK,
    OP_UNLOCK,
    OP_METADATA,
    OP_COUNT,
    OP_WALLET_IMPORT,
    OP_WALLET_GENERATE,
    OP_WALLET_ADDRESS,
    OP_SIGN_NOX_RECEIPT,
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

/// O(n^2) over 22 entries, in a const context, so it costs nothing at runtime
/// and the build fails on a repeat.
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
