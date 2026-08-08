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

use crate::crypto::lioness::Lioness;
use crate::sphinx::constants::PAYLOAD_KEY_SIZE;

/// Lay one LIONESS layer per hop, outermost last, so the first mix peels the
/// layer keyed to it and hands on something it cannot read.
pub fn seal_payload(payload: &mut [u8], keys: &[[u8; PAYLOAD_KEY_SIZE]]) -> Option<()> {
    for key in keys.iter().rev() {
        Lioness::new(key).encrypt_block(payload).ok()?;
    }
    Some(())
}
