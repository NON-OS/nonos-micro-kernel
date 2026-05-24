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
pub enum InterruptCode {
    UserSoftware,
    SupervisorSoftware,
    MachineSoftware,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
    Unknown(usize),
}

impl From<usize> for InterruptCode {
    fn from(code: usize) -> Self {
        match code {
            0 => Self::UserSoftware,
            1 => Self::SupervisorSoftware,
            3 => Self::MachineSoftware,
            4 => Self::UserTimer,
            5 => Self::SupervisorTimer,
            7 => Self::MachineTimer,
            8 => Self::UserExternal,
            9 => Self::SupervisorExternal,
            11 => Self::MachineExternal,
            n => Self::Unknown(n),
        }
    }
}
