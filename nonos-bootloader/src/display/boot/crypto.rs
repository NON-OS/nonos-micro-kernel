// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

#[derive(Clone, Copy, Default)]
pub struct BootCryptoState {
    pub hash_verified: bool,
    pub sig_verified: bool,
    pub zk_verified: bool,
    pub zk_present: bool,
    pub signature_valid: Option<bool>,
    pub hash: [u8; 32],
    pub pubkey: [u8; 32],
    pub kernel_hash: [u8; 32],
    pub signature_r: [u8; 32],
    pub signature_s: [u8; 32],
    pub zk_program_hash: [u8; 32],
}

impl BootCryptoState {
    pub const fn new() -> Self {
        Self {
            hash_verified: false,
            sig_verified: false,
            zk_verified: false,
            zk_present: false,
            signature_valid: None,
            hash: [0u8; 32],
            pubkey: [0u8; 32],
            kernel_hash: [0u8; 32],
            signature_r: [0u8; 32],
            signature_s: [0u8; 32],
            zk_program_hash: [0u8; 32],
        }
    }
}

pub fn show_crypto_verification(_state: &BootCryptoState) {}

pub fn animate_hash_reveal() {}
