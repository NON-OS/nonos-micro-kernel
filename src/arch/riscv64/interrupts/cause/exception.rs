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
pub enum ExceptionCode {
    InstructionMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadMisaligned,
    LoadAccessFault,
    StoreMisaligned,
    StoreAccessFault,
    UserEcall,
    SupervisorEcall,
    MachineEcall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown(usize),
}

impl From<usize> for ExceptionCode {
    fn from(code: usize) -> Self {
        match code {
            0 => Self::InstructionMisaligned,
            1 => Self::InstructionAccessFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadMisaligned,
            5 => Self::LoadAccessFault,
            6 => Self::StoreMisaligned,
            7 => Self::StoreAccessFault,
            8 => Self::UserEcall,
            9 => Self::SupervisorEcall,
            11 => Self::MachineEcall,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StorePageFault,
            n => Self::Unknown(n),
        }
    }
}
