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

use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::sign::sign_with;

use super::trailer::{append, ED25519_TAG, MLDSA65_TAG};
use crate::container::{encode_unsigned, Container, PackErr};

pub fn seal(c: &Container, ed_seed: &[u8], mldsa_seed: &[u8]) -> Result<Vec<u8>, PackErr> {
    let mut out = encode_unsigned(c);
    let digest = blake3::hash(&out);
    let inputs =
        [(ED25519_TAG, AlgId::Ed25519, ed_seed), (MLDSA65_TAG, AlgId::MlDsa65, mldsa_seed)];
    let mut sigs = Vec::with_capacity(inputs.len());
    for (tag, alg, seed) in inputs {
        let sig =
            sign_with(alg, seed, digest.as_bytes()).map_err(|e| PackErr::Crypto(e.to_string()))?;
        sigs.push((tag, sig));
    }
    append(&mut out, &sigs)?;
    Ok(out)
}
