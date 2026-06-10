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

use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_groth16::Proof;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_std::io::Cursor;

use crate::crypto::zk::groth16::error::Groth16Error;

const PROOF_BYTES: usize = 192;

pub(super) fn read_proof_bls12_381(blob: &[u8]) -> Result<Proof<Bls12_381>, Groth16Error> {
    if blob.len() != PROOF_BYTES {
        return Err(Groth16Error::SizeLimit("proof"));
    }
    let mut cur = Cursor::new(blob);
    let a = G1Affine::deserialize_with_mode(&mut cur, Compress::Yes, Validate::Yes)
        .map_err(|_| Groth16Error::Deserialize("proof.a"))?;
    let b = G2Affine::deserialize_with_mode(&mut cur, Compress::Yes, Validate::Yes)
        .map_err(|_| Groth16Error::Deserialize("proof.b"))?;
    let c = G1Affine::deserialize_with_mode(&mut cur, Compress::Yes, Validate::Yes)
        .map_err(|_| Groth16Error::Deserialize("proof.c"))?;
    if cur.position() as usize != blob.len() {
        return Err(Groth16Error::Deserialize("proof.trailing"));
    }
    Ok(Proof::<Bls12_381> { a, b, c })
}
