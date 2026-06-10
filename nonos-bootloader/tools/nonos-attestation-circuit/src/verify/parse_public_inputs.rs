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

use ark_bls12_381::Fr;
use ark_ff::PrimeField;

use super::constants::PUBLIC_INPUT_BYTES;
use super::validate_public_input_layout::validate_public_input_layout;

pub fn parse_public_inputs(bytes: &[u8]) -> Result<Vec<Fr>, String> {
    if bytes.len() != PUBLIC_INPUT_BYTES {
        return Err(format!("public input bytes {} != {}", bytes.len(), PUBLIC_INPUT_BYTES));
    }
    validate_public_input_layout(bytes)?;
    Ok(bytes.chunks_exact(32).map(Fr::from_be_bytes_mod_order).collect())
}
