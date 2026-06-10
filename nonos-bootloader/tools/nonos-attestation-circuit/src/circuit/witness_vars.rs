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

use ark_ff::PrimeField;
use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, fields::fp::FpVar, uint8::UInt8};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use super::types::NonosAttestationCircuit;
use crate::policy_tree::POLICY_TREE_DEPTH;

pub struct WitnessVars<F: PrimeField> {
    pub path: Vec<FpVar<F>>,
    pub dirs: Vec<Boolean<F>>,
    pub pcr: Vec<UInt8<F>>,
    pub hw: FpVar<F>,
}

impl<F: PrimeField> WitnessVars<F> {
    pub fn new(
        cs: ConstraintSystemRef<F>,
        c: &NonosAttestationCircuit<F>,
    ) -> Result<Self, SynthesisError> {
        let path = c.policy_path.as_ref().ok_or(SynthesisError::AssignmentMissing)?;
        let dirs = c.policy_dirs.as_ref().ok_or(SynthesisError::AssignmentMissing)?;
        if path.len() != POLICY_TREE_DEPTH || dirs.len() != POLICY_TREE_DEPTH {
            return Err(SynthesisError::AssignmentMissing);
        }
        let pcr = c.pcr_preimage.ok_or(SynthesisError::AssignmentMissing)?;
        let hw = c.hardware_attestation.ok_or(SynthesisError::AssignmentMissing)?;
        let path = path
            .iter()
            .map(|v| FpVar::new_witness(cs.clone(), || Ok(*v)))
            .collect::<Result<Vec<_>, _>>()?;
        let dirs = dirs
            .iter()
            .map(|d| Boolean::new_witness(cs.clone(), || Ok(*d)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            path,
            dirs,
            pcr: UInt8::new_witness_vec(cs.clone(), &pcr)?,
            hw: FpVar::new_witness(cs, || Ok(F::from(hw)))?,
        })
    }
}
