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

use core::marker::PhantomData;

use ark_ff::PrimeField;

use super::split_hash::split_hash;
use super::types::NonosAttestationCircuit;
use crate::constants::PCR_PREIMAGE_LEN;
use crate::policy_tree::PolicyWitness;

impl<F: PrimeField> NonosAttestationCircuit<F> {
    pub fn new(
        capsule_hash: [u8; 32],
        policy: PolicyWitness<F>,
        policy_epoch: u64,
        capability_mask: u64,
        commitment: [u8; 32],
        pcr_preimage: [u8; PCR_PREIMAGE_LEN],
        hardware_attestation: u64,
    ) -> Self {
        let (capsule_hash_hi, capsule_hash_lo) = split_hash(&capsule_hash);
        let (commitment_hi, commitment_lo) = split_hash(&commitment);
        Self {
            capsule_hash_hi: Some(capsule_hash_hi),
            capsule_hash_lo: Some(capsule_hash_lo),
            policy_root: Some(policy.root),
            policy_epoch: Some(F::from(policy_epoch)),
            capability_mask: Some(F::from(capability_mask)),
            commitment_hi: Some(commitment_hi),
            commitment_lo: Some(commitment_lo),
            policy_path: Some(policy.siblings),
            policy_dirs: Some(policy.dirs),
            pcr_preimage: Some(pcr_preimage),
            hardware_attestation: Some(hardware_attestation),
            phantom: PhantomData,
        }
    }
}
