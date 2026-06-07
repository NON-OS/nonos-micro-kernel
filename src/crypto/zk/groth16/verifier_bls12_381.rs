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

// BLS12-381 Groth16 verifier. The boot attestation and the per-capsule
// attestation proofs are produced over BLS12-381 (the bootloader verifies the
// kernel proof on the same curve). This lets the kernel verify a capsule's
// embedded proof at spawn, on the same curve as the rest of the trust chain,
// before the capsule is mapped or scheduled.

use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
use ark_ff::PrimeField;
use ark_groth16::{prepare_verifying_key, Groth16, Proof, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_std::io::Cursor;

use crate::crypto::zk::groth16::error::Groth16Error;
use crate::crypto::zk::groth16::{MAX_PROOF_BYTES, MAX_PUBLIC_INPUTS, MAX_VK_BYTES};

// Verify a Groth16 proof over BLS12-381. `public_inputs_be` is a flat buffer of
// 32-byte big-endian field elements, the layout the prover and the bootloader
// both use. Returns Ok(()) only when the proof verifies against the key and the
// supplied public inputs.
#[must_use = "verification result must be checked"]
pub fn groth16_verify_bls12_381(
    vk_bytes: &[u8],
    proof_blob: &[u8],
    public_inputs_be: &[u8],
) -> Result<(), Groth16Error> {
    if vk_bytes.len() > MAX_VK_BYTES {
        return Err(Groth16Error::SizeLimit("vk"));
    }
    if proof_blob.len() > MAX_PROOF_BYTES {
        return Err(Groth16Error::SizeLimit("proof"));
    }
    if public_inputs_be.len() % 32 != 0 {
        return Err(Groth16Error::InvalidPublicInput);
    }
    let inputs_count = public_inputs_be.len() / 32;
    if inputs_count > MAX_PUBLIC_INPUTS {
        return Err(Groth16Error::SizeLimit("public inputs"));
    }

    let vk = VerifyingKey::<Bls12_381>::deserialize_with_mode(
        &mut Cursor::new(vk_bytes),
        Compress::Yes,
        Validate::Yes,
    )
    .map_err(|_| Groth16Error::Deserialize("vk"))?;

    if inputs_count != vk.gamma_abc_g1.len().saturating_sub(1) {
        return Err(Groth16Error::InvalidPublicInput);
    }

    let mut cur = Cursor::new(proof_blob);
    let a = G1Affine::deserialize_with_mode(&mut cur, Compress::Yes, Validate::Yes)
        .map_err(|_| Groth16Error::Deserialize("proof.a"))?;
    let b = G2Affine::deserialize_with_mode(&mut cur, Compress::Yes, Validate::Yes)
        .map_err(|_| Groth16Error::Deserialize("proof.b"))?;
    let c = G1Affine::deserialize_with_mode(&mut cur, Compress::Yes, Validate::Yes)
        .map_err(|_| Groth16Error::Deserialize("proof.c"))?;
    let proof = Proof::<Bls12_381> { a, b, c };

    let mut inputs = alloc::vec::Vec::with_capacity(inputs_count);
    for chunk in public_inputs_be.chunks_exact(32) {
        inputs.push(Fr::from_be_bytes_mod_order(chunk));
    }

    let pvk = prepare_verifying_key(&vk);
    match Groth16::<Bls12_381>::verify_proof(&pvk, &proof, &inputs) {
        Ok(true) => Ok(()),
        Ok(false) => Err(Groth16Error::VerifyFailed),
        Err(_) => Err(Groth16Error::VerifyFailed),
    }
}
