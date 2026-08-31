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

impl Mode {
    pub fn name(self) -> &'static [u8] {
        match self {
            Mode::Arcade => b"Arcade",
            Mode::Classic => b"Classic",
            Mode::TimeAttack => b"Time Attack",
            Mode::Zen => b"Zen",
        }
    }

    pub fn blurb(self) -> &'static [u8] {
        match self {
            Mode::Arcade => b"Three lives, the pace climbs with every bite",
            Mode::Classic => b"One life, one speed, hard walls",
            Mode::TimeAttack => b"Ninety seconds, one life, score all you can",
            Mode::Zen => b"Edges wrap and a crash costs you nothing",
        }
    }
}
