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

// Human-decoded transaction shown on the clear-signing screen. Never the raw
// bytes: the signer only ever receives what the user has seen decoded here.
#[derive(Clone)]
pub struct DecodedTx {
    pub chain_id: u64,
    pub to: [u8; 20],
    pub value_wei: u128,
    pub function: &'static [u8],
    pub token: Option<[u8; 20]>,
    pub token_amount_wei: u128,
    pub recipient: Option<[u8; 20]>,
    pub max_fee_wei: u128,
    // true for approve()/setApprovalForAll: needs the approval-safety gate.
    pub is_approval: bool,
    pub infinite_approval: bool,
    // set when the tx does something we cannot fully decode.
    pub warnings: Vec<&'static [u8]>,
    pub raw_len: usize,
}

// One balance change in a simulated state diff.
#[derive(Clone, Copy)]
pub struct BalanceDelta {
    pub token: [u8; 20],
    pub is_native: bool,
    pub delta_wei: i128,
}

// Result of simulating a tx against a forked head.
#[derive(Clone)]
pub struct StateDiff {
    pub deltas: Vec<BalanceDelta>,
    pub approvals_granted: Vec<[u8; 20]>,
    pub reverted: bool,
    pub revert_reason: &'static [u8],
}
