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

use super::mode::Mode;

#[derive(Clone, Copy)]
pub struct RunRecord {
    pub score: u32,
    pub mode: Mode,
    pub length: u16,
    pub seq: u32,
}

pub const MAX_RUNS: usize = 10;

impl RunRecord {
    pub fn new(score: u32, mode: Mode, length: u16, seq: u32) -> Self {
        RunRecord { score, mode, length, seq }
    }
}
