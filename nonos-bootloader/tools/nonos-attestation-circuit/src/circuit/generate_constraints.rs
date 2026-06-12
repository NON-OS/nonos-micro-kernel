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

use ark_crypto_primitives::sponge::Absorb;
use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use super::enforce_basics::enforce_basics;
use super::enforce_epoch::enforce_epoch;
use super::enforce_policy::enforce_policy;
use super::public_vars::PublicVars;
use super::types::NonosAttestationCircuit;
use super::witness_vars::WitnessVars;

impl<F: PrimeField + Absorb> ConstraintSynthesizer<F> for NonosAttestationCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let public = PublicVars::new(cs.clone(), &self)?;
        let witness = WitnessVars::new(cs.clone(), &self)?;
        enforce_basics(cs.clone(), &public, &witness)?;
        enforce_epoch(cs.clone(), &public)?;
        enforce_policy(cs, &public, &witness)?;
        Ok(())
    }
}
