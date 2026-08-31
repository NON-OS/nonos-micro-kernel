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

pub const COUNT: usize = 5;

// score threshold, name, wall cells carried at this level.
const TABLE: [(u32, &[u8], usize); COUNT] = [
    (0, b"Open Field", 0),
    (500, b"Neon Grid", 6),
    (1200, b"The Lattice", 12),
    (2200, b"Chokepoint", 18),
    (3600, b"Deep Stack", 26),
];

pub fn index_for(score: u32) -> usize {
    let mut level = 0;
    for (i, entry) in TABLE.iter().enumerate() {
        if score >= entry.0 {
            level = i;
        }
    }
    level
}

pub fn name(level: usize) -> &'static [u8] {
    TABLE[level.min(COUNT - 1)].1
}

pub fn walls(level: usize) -> usize {
    TABLE[level.min(COUNT - 1)].2
}

pub fn threshold(level: usize) -> u32 {
    TABLE[level.min(COUNT - 1)].0
}

pub fn next_threshold(level: usize) -> Option<u32> {
    if level + 1 >= COUNT {
        return None;
    }
    Some(TABLE[level + 1].0)
}
