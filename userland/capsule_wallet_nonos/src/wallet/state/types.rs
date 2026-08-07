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

use crate::wallet::net::NetStatus;
use alloc::vec::Vec;

pub const MAX_RAILS: usize = 8;
pub const MAX_STAKE: u32 = 18204;
pub const VIEW_HOME: u8 = 0;
pub const VIEW_RECEIVE: u8 = 1;
pub const VIEW_SEND: u8 = 2;
pub const VIEW_PROOF: u8 = 3;
// New hardened-wallet + private-swap + NOX screens.
pub const VIEW_SIGN: u8 = 4;
pub const VIEW_APPROVALS: u8 = 5;
pub const VIEW_SHIELD: u8 = 6;
pub const VIEW_UNSHIELD: u8 = 7;
pub const VIEW_SHIELDED: u8 = 8;
pub const VIEW_NOX: u8 = 9;
pub const VIEW_SWAP: u8 = 10;
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
    /// Which token the trade pays out of, as an index into the token list.
    pub swap_from: u8,
    /// Which token the trade buys.
    pub swap_to: u8,
    /// Amount to pay, in the paying token's smallest unit.
    pub swap_in: u128,
    /// What the pool last said this trade returns.
    pub swap_quote: crate::wallet::swap::Quote,
    /// Slippage the reader will accept, in hundredths of a percent.
    pub swap_slippage_bps: u32,
    /// Zero while the router still needs an allowance, one once it has one.
    pub swap_step: u8,
    /// How many digits the reader has typed, so a correction knows what to
    /// take back.
    pub swap_digits: u32,
    /// Digits typed after the point.
    pub swap_places: u32,
    /// Whether the reader has started a fraction.
    pub swap_point: bool,
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
    // NOX to stake, in wei. Held at chain precision rather than whole tokens
    // so any amount can be typed, fractions included, with no ceiling beyond
    // what the wallet actually holds.
    pub stake_amount: u128,
    // Decimal entry for the amount above: digits typed, places after the
    // point, and whether a point has been started.
    pub stake_digits: u32,
    pub stake_places: u32,
    pub stake_point: bool,
    // Which staked position the Unstake tab acts on. The contract closes a
    // position by index, not by amount.
    pub stake_position: u64,
    // Chosen lock term, as an index into the contract lock table. Zero is no
    // lock, which is what plain stake() does.
    pub stake_lock: u8,
    // Two-step staking: 0 = needs the approve, 1 = ready to stake. Advances once
    // the approve broadcasts and resets after the stake.
    pub stake_step: u8,
    // Which asset the send screen transfers: 0 = ETH, 1 = NOX.
    pub send_token: u8,
    // Private-key import entry. The typed hex never renders and is wiped the
    // moment the key is handed to the keyring or the field is cancelled.
    pub import_active: bool,
    pub import_hex: [u8; 64],
    pub import_len: usize,
    // One-time mnemonic backup: the word indices exist here only while the
    // backup screen is showing and are volatile-wiped the moment the user
    // confirms. They are never persisted, logged, or kept past that screen.
    pub backup_active: bool,
    pub backup_words: [u16; 24],
    pub backup_count: u8,
    // Recovery-phrase entry: typed words, space separated, shown while typing
    // so the user can check them, wiped on submit or cancel.
    pub recover_active: bool,
    pub recover_buf: [u8; 240],
    pub recover_len: usize,
    // Private-key reveal for backup/export. Held only while shown, wiped the
    // moment it is hidden. `export_hex` is the 0x-prefixed key when revealed.
    pub export_active: bool,
    pub export_hex: [u8; 66],
    // Live NOX token and staking readout from mainnet eth_call.
    pub nox: crate::wallet::nox::NoxStatus,
    // Which field the incremental probe refreshes next. One network read per
    // tick keeps the UI responsive instead of blocking on a burst of them.
    pub probe_step: u8,
    // The framebuffer width recorded on the last paint, so pointer handlers can
    // hit-test the same width-relative layout the screens draw.
    pub view_w: u32,
    // Framebuffer height recorded on the last paint, so the pointer handlers
    // hit-test the same height-relative layout the screens draw.
    pub view_h: u32,
    // Local shielded UTXO set, reconstructed from the note secrets.
    pub notes: crate::wallet::shield::notes::NoteStore,
}
