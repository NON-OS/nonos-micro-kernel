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

extern crate alloc;
use super::types_category::CircuitCategory;
use super::types_verify::{compute_circuit_signing_data, verify_circuit_signature};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct CircuitEntry {
    pub program_hash: [u8; 32],
    pub vk_bytes: &'static [u8],
    pub name: &'static str,
    pub version: &'static str,
    pub permissions: u32,
    pub category: CircuitCategory,
    pub signature: Option<&'static [u8; 64]>,
    pub signer: Option<&'static [u8; 32]>,
}

impl CircuitEntry {
    pub fn compute_signing_data(&self) -> [u8; 32] {
        compute_circuit_signing_data(
            &self.program_hash,
            self.vk_bytes,
            self.name,
            self.version,
            self.permissions,
            self.category,
        )
    }
    pub fn has_valid_signature(&self) -> bool {
        match (self.signature, self.signer) {
            (Some(sig), Some(pk)) => {
                verify_circuit_signature(&self.compute_signing_data(), sig, pk)
            }
            _ => false,
        }
    }
    pub fn is_core_signed(&self) -> bool {
        self.category == CircuitCategory::Core && self.has_valid_signature()
    }
}

#[derive(Debug, Clone)]
pub struct DynamicCircuitEntry {
    pub program_hash: [u8; 32],
    pub vk_bytes: Vec<u8>,
    pub name: Vec<u8>,
    pub permissions: u32,
    pub category: CircuitCategory,
    pub loaded_at: u64,
}
