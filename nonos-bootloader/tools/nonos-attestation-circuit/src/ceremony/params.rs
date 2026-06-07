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

use super::error::CeremonyError;
use ark_bls12_381::Bls12_381;
use ark_groth16::ProvingKey;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};

pub struct CeremonyParams {
    pub pk: ProvingKey<Bls12_381>,
    pub round: u32,
    pub params_hash: [u8; 32],
}

impl CeremonyParams {
    pub fn serialize(&self) -> Result<Vec<u8>, CeremonyError> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.round.to_le_bytes());
        buf.extend_from_slice(&self.params_hash);
        self.pk
            .serialize_with_mode(&mut buf, Compress::Yes)
            .map_err(|e| CeremonyError::SerializationError(e.to_string()))?;
        Ok(buf)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, CeremonyError> {
        if data.len() < 36 {
            return Err(CeremonyError::InvalidPreviousParams);
        }
        let round = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let mut params_hash = [0u8; 32];
        params_hash.copy_from_slice(&data[4..36]);
        let pk = ProvingKey::<Bls12_381>::deserialize_with_mode(
            &data[36..],
            Compress::Yes,
            Validate::Yes,
        )
        .map_err(|e| CeremonyError::SerializationError(e.to_string()))?;
        Ok(Self { pk, round, params_hash })
    }
}
