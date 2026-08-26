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

use super::State;

fn fold(byte: u8) -> u8 {
    if byte.is_ascii_uppercase() {
        byte + 32
    } else {
        byte
    }
}

impl State {
    // Case-insensitive substring, true on an empty query so `filtered` can join
    // it to the chip predicate unconditionally rather than branching per row.
    pub fn query_matches(&self, name: &[u8]) -> bool {
        let q = self.query();
        if q.is_empty() {
            return true;
        }
        if name.len() < q.len() {
            return false;
        }
        (0..=name.len() - q.len())
            .any(|i| name[i..].iter().zip(q).all(|(a, b)| fold(*a) == fold(*b)))
    }
}
