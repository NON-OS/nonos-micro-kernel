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

use super::surb_types::Surb;
use crate::crypto::fill_random;

pub(super) fn create(existing: &[Surb], owner: u32) -> Option<(u32, [u8; 32])> {
    let mut tries = 0;
    while tries < 8 {
        tries += 1;
        let mut seed = [0u8; 32];
        fill_random(&mut seed).ok()?;
        let id = u32::from_le_bytes([seed[0], seed[1], seed[2], seed[3]]);
        if id != 0 && !exists(existing, owner, id) {
            return Some((id, seed));
        }
    }
    None
}

fn exists(existing: &[Surb], owner: u32, id: u32) -> bool {
    existing.iter().any(|s| s.owner == owner && s.id == id)
}
