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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BtiGuard {
    None = 0,
    C = 1,
    J = 2,
    Jc = 3,
}

impl BtiGuard {
    pub fn instruction(&self) -> u32 {
        match self {
            BtiGuard::None => 0xD503201F,
            BtiGuard::C => 0xD503245F,
            BtiGuard::J => 0xD503249F,
            BtiGuard::Jc => 0xD50324DF,
        }
    }
}
