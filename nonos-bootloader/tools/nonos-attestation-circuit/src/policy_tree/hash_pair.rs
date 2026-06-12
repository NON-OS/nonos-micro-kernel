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

use ark_crypto_primitives::{crh::poseidon::TwoToOneCRH, crh::TwoToOneCRHScheme, sponge::Absorb};
use ark_ff::PrimeField;

use super::params::params;

pub fn hash_pair<F: PrimeField + Absorb>(left: F, right: F) -> Result<F, String> {
    TwoToOneCRH::<F>::compress(&params::<F>(), left, right)
        .map_err(|e| format!("poseidon node: {e:?}"))
}
