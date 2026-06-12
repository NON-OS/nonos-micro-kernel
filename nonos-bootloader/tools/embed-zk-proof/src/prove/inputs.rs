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

use anyhow::{Context, Result};
use ark_bls12_381::Fr;
use ark_ff::{BigInteger, PrimeField};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};

use super::circuit::{build_circuit, CircuitParams};

pub fn extract_public_inputs(params: &CircuitParams) -> Result<Vec<u8>> {
    let circuit = build_circuit(params).map_err(anyhow::Error::msg)?;
    let cs = ConstraintSystem::<Fr>::new_ref();

    circuit
        .generate_constraints(cs.clone())
        .with_context(|| "Failed to synthesize circuit for public inputs")?;

    let elements: Vec<Fr> = cs
        .borrow()
        .ok_or_else(|| anyhow::anyhow!("Failed to borrow constraint system"))?
        .instance_assignment
        .iter()
        .skip(1)
        .cloned()
        .collect();

    let mut bytes = Vec::with_capacity(elements.len() * 32);
    for fe in &elements {
        let be_bytes = fe.into_bigint().to_bytes_be();
        let padding = 32 - be_bytes.len();
        bytes.extend(std::iter::repeat_n(0u8, padding));
        bytes.extend_from_slice(&be_bytes);
    }

    Ok(bytes)
}
