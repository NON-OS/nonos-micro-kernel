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
pub enum Mode {
    Arcade,
    Classic,
    TimeAttack,
    Zen,
}

pub const ALL: [Mode; 4] = [Mode::Arcade, Mode::Classic, Mode::TimeAttack, Mode::Zen];

pub const TIME_ATTACK_MS: i64 = 90_000;

impl Mode {
    pub fn lives(self) -> u8 {
        match self {
            Mode::Arcade => 3,
            _ => 1,
        }
    }

    // Classic is the one fixed-pace mode; the rest ride the difficulty curve.
    pub fn speeds_up(self) -> bool {
        self != Mode::Classic
    }

    pub fn time_limit_ms(self) -> i64 {
        match self {
            Mode::TimeAttack => TIME_ATTACK_MS,
            _ => 0,
        }
    }

    // Zen wraps whatever the toggle says; Classic refuses to wrap at all.
    pub fn forces_wrap(self) -> bool {
        self == Mode::Zen
    }

    pub fn hard_walls(self) -> bool {
        self == Mode::Classic
    }

    // In Zen a fatal move is simply refused, so the run never ends.
    pub fn is_lethal(self) -> bool {
        self != Mode::Zen
    }
}
