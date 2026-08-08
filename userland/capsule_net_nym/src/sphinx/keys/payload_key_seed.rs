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

use super::super::constants::PAYLOAD_KEY_SEED_SIZE;
use super::offsets::PAYLOAD_KEY_AT;
use super::types::ExpandedSharedSecret;

impl ExpandedSharedSecret {
    /// The v2 seed. It deliberately overlaps the first 16 bytes of the legacy
    /// key: same expansion, read either as the key itself or as a seed to
    /// stretch, which is what distinguishes the two packet versions.
    pub fn payload_key_seed(&self) -> [u8; PAYLOAD_KEY_SEED_SIZE] {
        let mut seed = [0u8; PAYLOAD_KEY_SEED_SIZE];
        seed.copy_from_slice(&self.0[PAYLOAD_KEY_AT..PAYLOAD_KEY_AT + PAYLOAD_KEY_SEED_SIZE]);
        seed
    }
}
