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

use crate::snake::state::Mode;

// The stored mode is its own byte, decoupled from the enum's declaration order
// so a future variant cannot silently reinterpret an old record.
pub fn mode_byte(mode: Mode) -> u8 {
    match mode {
        Mode::Arcade => 0,
        Mode::Classic => 1,
        Mode::TimeAttack => 2,
        Mode::Zen => 3,
    }
}

pub fn mode_of(byte: u8) -> Result<Mode, &'static str> {
    match byte {
        0 => Ok(Mode::Arcade),
        1 => Ok(Mode::Classic),
        2 => Ok(Mode::TimeAttack),
        3 => Ok(Mode::Zen),
        _ => Err("snake store mode"),
    }
}
