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
pub enum MteMode {
    Disabled,
    Synchronous,
    Asynchronous,
    Asymmetric,
}

impl MteMode {
    pub const fn tcf(self) -> u64 {
        match self {
            MteMode::Disabled => 0b00,
            MteMode::Synchronous => 0b01,
            MteMode::Asynchronous => 0b10,
            MteMode::Asymmetric => 0b11,
        }
    }
}
