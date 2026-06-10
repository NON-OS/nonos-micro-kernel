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

use super::types::NonosAttestationCircuit;
use crate::constants::{MIN_HW_LEVEL, MIN_PCR_ENTROPY_BYTES, PCR_PREIMAGE_LEN};
use crate::policy_tree::{POLICY_EPOCH, POLICY_TREE_DEPTH};

impl<F: PrimeField> Default for NonosAttestationCircuit<F> {
    fn default() -> Self {
        let mut pcr = [0u8; PCR_PREIMAGE_LEN];
        for (i, byte) in pcr.iter_mut().take(MIN_PCR_ENTROPY_BYTES).enumerate() {
            *byte = (i as u8).wrapping_add(1);
        }
        Self {
            capsule_hash_hi: Some(F::from(1u64)),
            capsule_hash_lo: Some(F::from(2u64)),
            policy_root: Some(F::from(3u64)),
            policy_epoch: Some(F::from(POLICY_EPOCH)),
            capability_mask: Some(F::from(5u64)),
            commitment_hi: Some(F::from(6u64)),
            commitment_lo: Some(F::from(7u64)),
            policy_path: Some(vec![F::from(8u64); POLICY_TREE_DEPTH]),
            policy_dirs: Some(vec![false; POLICY_TREE_DEPTH]),
            pcr_preimage: Some(pcr),
            hardware_attestation: Some(MIN_HW_LEVEL + 1),
            phantom: PhantomData,
        }
    }
}
