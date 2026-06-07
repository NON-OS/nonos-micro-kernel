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

use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, ProvingKey};
use ark_serialize::{CanonicalSerialize, Compress};
use ark_snark::SNARK;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use nonos_attestation_circuit::{NonosAttestationCircuit, GROTH16_PROOF_SIZE};

pub fn proof(
    pk: &ProvingKey<Bls12_381>,
    circuit: NonosAttestationCircuit<Fr>,
    seed: &[u8],
) -> Result<Vec<u8>, String> {
    let seed_hash = blake3::hash(seed);
    let seed_u64 = u64::from_le_bytes(seed_hash.as_bytes()[..8].try_into().map_err(|_| "seed")?);
    let mut rng = StdRng::seed_from_u64(seed_u64);
    let proof =
        Groth16::<Bls12_381>::prove(pk, circuit, &mut rng).map_err(|e| format!("prove: {e}"))?;
    let mut bytes = Vec::new();
    proof
        .serialize_with_mode(&mut bytes, Compress::Yes)
        .map_err(|e| format!("serialize proof: {e}"))?;
    if bytes.len() != GROTH16_PROOF_SIZE {
        return Err(format!("proof size {} != {}", bytes.len(), GROTH16_PROOF_SIZE));
    }
    Ok(bytes)
}
