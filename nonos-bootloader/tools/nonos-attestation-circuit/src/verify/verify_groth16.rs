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

use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_snark::SNARK;

pub fn verify_groth16(
    vk: &VerifyingKey<Bls12_381>,
    proof: &[u8],
    inputs: &[Fr],
) -> Result<bool, String> {
    let proof = Proof::<Bls12_381>::deserialize_with_mode(proof, Compress::Yes, Validate::Yes)
        .map_err(|e| format!("read proof: {e}"))?;
    Groth16::<Bls12_381>::verify(vk, inputs, &proof).map_err(|e| format!("verify: {e}"))
}
