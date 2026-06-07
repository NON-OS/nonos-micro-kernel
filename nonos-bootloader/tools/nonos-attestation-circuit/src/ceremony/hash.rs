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

use super::constants::DS_CEREMONY;
use ark_bls12_381::Bls12_381;
use ark_groth16::ProvingKey;
use ark_serialize::{CanonicalSerialize, Compress};

pub fn hash_params(pk: &ProvingKey<Bls12_381>) -> [u8; 32] {
    let mut buf = Vec::new();
    pk.serialize_with_mode(&mut buf, Compress::Yes).unwrap();
    let mut hasher = blake3::Hasher::new_derive_key(DS_CEREMONY);
    hasher.update(&buf);
    *hasher.finalize().as_bytes()
}
