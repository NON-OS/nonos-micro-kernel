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

pub const MIN_KERNEL_SIZE: usize = 64 + 1024;

pub const SIGNATURE_SIZE: usize = 64;

#[derive(Debug, Clone)]
pub struct CryptoVerifyResult {
    pub signature_valid: bool,
    pub kernel_hash_preview: [u8; 8],
    pub kernel_hash_full: [u8; 32],
    pub kernel_code_size: usize,
    pub signature_present: bool,
    /// The kernel's STARK self-attestation verified against the enrolled root.
    #[cfg(feature = "stark-kernel-attest")]
    pub stark_attested: bool,
}

impl Default for CryptoVerifyResult {
    fn default() -> Self {
        Self {
            signature_valid: false,
            kernel_hash_preview: [0u8; 8],
            kernel_hash_full: [0u8; 32],
            kernel_code_size: 0,
            signature_present: false,
            #[cfg(feature = "stark-kernel-attest")]
            stark_attested: false,
        }
    }
}

impl CryptoVerifyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
