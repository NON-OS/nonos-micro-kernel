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
use crate::wallet::net::NetStatus;

pub const MAX_RAILS: usize = 8;
pub const VIEW_HOME: u8 = 0; pub const VIEW_RECEIVE: u8 = 1; pub const VIEW_SEND: u8 = 2; pub const VIEW_PROOF: u8 = 3;
// New hardened-wallet + private-swap + NOX screens.
pub const VIEW_SIGN: u8 = 4; pub const VIEW_APPROVALS: u8 = 5; pub const VIEW_SHIELD: u8 = 6;
pub const VIEW_UNSHIELD: u8 = 7; pub const VIEW_SHIELDED: u8 = 8; pub const VIEW_NOX: u8 = 9;
pub const SEND_FIELD_TO: u8 = 0;
pub const SEND_FIELD_AMOUNT: u8 = 1;
pub const SEND_FIELD_NONCE: u8 = 2;

#[derive(Clone, Copy)]
pub struct Rail {
    pub symbol: [u8; 8],
    pub symbol_len: u8,
    pub family: u8,
    pub status: u16,
    pub flags: u32,
    pub chain_id: u64,
    pub contract: [u8; 20],
}

pub struct State {
    pub keyring_port: u32,
    pub owner_pid: u32,
    pub wallet_id: u32,
    pub address: [u8; 20],
    pub address_ready: bool,
    pub balance_ready: bool,
    pub balance_wei: [u8; 32],
    pub nonce_ready: bool,
    pub live_nonce: u64,
    pub fee_ready: bool,
    pub fee_wei: u64,
    pub view: u8,
    pub send_focus: u8,
    pub send_to_hex: [u8; 40],
    pub send_to_len: usize,
    pub send_amount_milli_eth: u32,
    pub send_nonce: u64,
    pub tx_hash: [u8; 32],
    pub tx_len: u32,
    pub tx_raw: Vec<u8>,
    pub tx_ready: bool,
    pub tx_kind: &'static [u8],
    pub broadcast_ready: bool,
    pub broadcast_hash: [u8; 32],
    pub receipt_ready: bool,
    pub receipt_ok: bool,
    pub proof_count: u8,
    pub proof_eth_hash: [u8; 32],
    pub proof_eth_len: u32,
    pub proof_nox_hash: [u8; 32],
    pub proof_nox_len: u32,
    pub net: NetStatus,
    pub rails: [Rail; MAX_RAILS],
    pub rail_count: usize,
    pub status: &'static [u8],
    // Live input readout, refreshed on each discrete key/button event so the
    // running UI can show whether pointer clicks actually reach the capsule.
    pub in_count: u32,
    pub in_kind: u32,
    pub in_x: i32,
    pub in_y: i32,
    // UI selections driven by pointer clicks: fee tier (0..2), stake/unstake
    // mode (0/1), proof filter (0..2), and the amount ETH/USD toggle.
    pub fee_tier: u8,
    pub stake_unstake: u8,
    pub proof_filter: u8,
    pub usd_mode: bool,
    pub light_mode: bool,
    // Header controls: which dropdown/overlay is open (0 none, 1 command,
    // 2 messages, 3 account) and whether the wallet is locked.
    pub panel: u8,
    pub locked: bool,
    pub account: u8,
    // Local shielded UTXO set, reconstructed from the note secrets.
    pub notes: crate::wallet::shield::notes::NoteStore,
}
