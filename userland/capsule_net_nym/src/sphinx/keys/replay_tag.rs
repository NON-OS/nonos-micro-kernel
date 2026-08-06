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

use super::super::constants::{BLINDING_FACTOR_SIZE, REPLAY_TAG_SIZE};
use super::offsets::BLINDING_FACTOR_AT;
use super::types::ExpandedSharedSecret;

impl ExpandedSharedSecret {
    pub fn replay_tag(&self) -> [u8; REPLAY_TAG_SIZE] {
        let at = BLINDING_FACTOR_AT + BLINDING_FACTOR_SIZE;
        let mut tag = [0u8; REPLAY_TAG_SIZE];
        tag.copy_from_slice(&self.0[at..at + REPLAY_TAG_SIZE]);
        tag
    }
}
