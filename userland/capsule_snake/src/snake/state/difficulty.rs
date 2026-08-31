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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Insane,
}

pub const ALL: [Difficulty; 4] =
    [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard, Difficulty::Insane];

// name, start interval, floor interval, speed-up per food. Normal is the
// constant set the capsule shipped with, so the default feel is unchanged.
const TABLE: [(&[u8], i64, i64, i64); 4] = [
    (b"Easy", 200, 120, 3),
    (b"Normal", 160, 80, 4),
    (b"Hard", 120, 60, 5),
    (b"Insane", 90, 45, 6),
];

impl Difficulty {
    pub fn index(self) -> usize {
        match self {
            Difficulty::Easy => 0,
            Difficulty::Normal => 1,
            Difficulty::Hard => 2,
            Difficulty::Insane => 3,
        }
    }

    pub fn name(self) -> &'static [u8] {
        TABLE[self.index()].0
    }

    pub fn start_ms(self) -> i64 {
        TABLE[self.index()].1
    }

    pub fn floor_ms(self) -> i64 {
        TABLE[self.index()].2
    }

    pub fn speedup_ms(self) -> i64 {
        TABLE[self.index()].3
    }
}
