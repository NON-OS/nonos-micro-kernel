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

use anyhow::{bail, Context, Result};
use ark_bls12_381::Bls12_381;
use ark_groth16::{Groth16, Proof, ProvingKey};
use ark_serialize::{CanonicalSerialize, Compress};
use ark_snark::SNARK;
use ark_std::rand::{rngs::StdRng, SeedableRng};

use super::circuit::{build_circuit, CircuitParams};

pub fn generate_proof(pk: &ProvingKey<Bls12_381>, params: &CircuitParams) -> Result<Vec<u8>> {
    let circuit = build_circuit(params).map_err(anyhow::Error::msg)?;

    let seed_hash = blake3::hash(&params.capsule_commitment);
    let mut seed = [0u8; 8];
    seed.copy_from_slice(&seed_hash.as_bytes()[..8]);
    let seed_u64 = u64::from_le_bytes(seed);
    let mut rng = StdRng::seed_from_u64(seed_u64);

    let proof: Proof<Bls12_381> = Groth16::<Bls12_381>::prove(pk, circuit, &mut rng)
        .with_context(|| "Groth16 proof generation failed")?;

    let mut proof_bytes = Vec::new();
    proof
        .serialize_with_mode(&mut proof_bytes, Compress::Yes)
        .with_context(|| "Failed to serialize proof")?;

    if proof_bytes.len() != 192 {
        bail!("Unexpected proof size: {} (expected 192)", proof_bytes.len());
    }

    Ok(proof_bytes)
}
