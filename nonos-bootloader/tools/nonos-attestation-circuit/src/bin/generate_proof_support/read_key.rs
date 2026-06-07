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

use std::{fs, path::Path};

use ark_bls12_381::Bls12_381;
use ark_groth16::ProvingKey;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};

pub fn read_key(path: &Path) -> Result<ProvingKey<Bls12_381>, String> {
    let bytes = fs::read(path).map_err(|e| format!("read proving key: {e}"))?;
    ProvingKey::<Bls12_381>::deserialize_with_mode(&bytes[..], Compress::Yes, Validate::Yes)
        .map_err(|e| format!("deserialize proving key: {e}"))
}
