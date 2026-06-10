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

use ark_crypto_primitives::{
    crh::{
        poseidon::constraints::{CRHGadget, CRHParametersVar, TwoToOneCRHGadget},
        CRHSchemeGadget, TwoToOneCRHSchemeGadget,
    },
    sponge::Absorb,
};
use ark_ff::PrimeField;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar, select::CondSelectGadget};
use ark_relations::{
    ns,
    r1cs::{ConstraintSystemRef, SynthesisError},
};

use super::public_vars::PublicVars;
use super::witness_vars::WitnessVars;
use crate::policy_tree::params;

pub fn enforce_policy<F: PrimeField + Absorb>(
    cs: ConstraintSystemRef<F>,
    public: &PublicVars<F>,
    witness: &WitnessVars<F>,
) -> Result<(), SynthesisError> {
    let p = params::<F>();
    let pv = CRHParametersVar::<F>::new_constant(ns!(cs, "poseidon"), &p)?;
    let input = [public.capsule_hi.clone(), public.capsule_lo.clone(), public.caps.clone()];
    let mut node = CRHGadget::<F>::evaluate(&pv, &input)?;
    for (sibling, dir) in witness.path.iter().zip(witness.dirs.iter()) {
        let left = FpVar::conditionally_select(dir, sibling, &node)?;
        let right = FpVar::conditionally_select(dir, &node, sibling)?;
        node = TwoToOneCRHGadget::<F>::compress(&pv, &left, &right)?;
    }
    node.enforce_equal(&public.policy_root)
}
