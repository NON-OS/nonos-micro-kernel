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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Extension {
    I,
    M,
    A,
    F,
    D,
    G,
    Q,
    C,
    B,
    V,
    H,
    S,
    U,
    Zicsr,
    Zifencei,
    Zicntr,
    Zihpm,
    Zkr,
    Zkn,
    Zks,
    Zvl128b,
    Zvl256b,
}

impl Extension {
    pub(super) fn bit(&self) -> Option<usize> {
        match self {
            Extension::I => Some(8),
            Extension::M => Some(12),
            Extension::A => Some(0),
            Extension::F => Some(5),
            Extension::D => Some(3),
            Extension::G => None,
            Extension::Q => Some(16),
            Extension::C => Some(2),
            Extension::B => Some(1),
            Extension::V => Some(21),
            Extension::H => Some(7),
            Extension::S => Some(18),
            Extension::U => Some(20),
            _ => None,
        }
    }
}
