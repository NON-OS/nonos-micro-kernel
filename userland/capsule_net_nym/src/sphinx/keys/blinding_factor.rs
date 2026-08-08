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

use super::super::constants::BLINDING_FACTOR_SIZE;
use super::offsets::BLINDING_FACTOR_AT;
use super::types::ExpandedSharedSecret;

impl ExpandedSharedSecret {
    pub fn blinding_factor(&self) -> [u8; BLINDING_FACTOR_SIZE] {
        let mut bf = [0u8; BLINDING_FACTOR_SIZE];
        bf.copy_from_slice(&self.0[BLINDING_FACTOR_AT..BLINDING_FACTOR_AT + BLINDING_FACTOR_SIZE]);
        bf
    }
}
