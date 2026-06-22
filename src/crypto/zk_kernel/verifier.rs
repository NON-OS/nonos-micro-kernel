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

use spin::Mutex;

mod stats;
mod verify_commitment;
mod verify_plonk;
mod verify_range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZkResult {
    Valid,
    Invalid,
    MalformedProof,
    UnsupportedProofType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ProofSystem {
    Range = 3,
    Equality = 4,
    Membership = 5,
    Pedersen = 6,
    Plonk = 7,
}

pub struct KernelZkVerifier {
    pub proofs_verified: u64,
    pub proofs_valid: u64,
    pub proofs_invalid: u64,
}

impl KernelZkVerifier {
    pub const fn new() -> Self {
        Self { proofs_verified: 0, proofs_valid: 0, proofs_invalid: 0 }
    }
}

pub static KERNEL_ZK_VERIFIER: Mutex<KernelZkVerifier> = Mutex::new(KernelZkVerifier::new());
