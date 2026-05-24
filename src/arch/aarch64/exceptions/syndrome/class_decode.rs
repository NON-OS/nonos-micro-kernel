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

use super::class::ExceptionClass;

impl From<u8> for ExceptionClass {
    fn from(ec: u8) -> Self {
        match ec {
            0b000000 => Self::Unknown,
            0b000001 => Self::WfeWfi,
            0b000011 => Self::Cp15Mcr,
            0b000100 => Self::Cp15Mcrr,
            0b000101 => Self::Cp14Mcr,
            0b000110 => Self::Cp14Ldc,
            0b000111 => Self::FpAccess,
            0b001100 => Self::Cp14Mrrc,
            0b001101 => Self::BranchTarget,
            0b001110 => Self::IllegalState,
            0b010001 => Self::Svc32,
            0b010101 => Self::Svc64,
            0b010110 => Self::Hvc64,
            0b010111 => Self::Smc64,
            0b011000 => Self::SysReg,
            0b011001 => Self::SveAccess,
            0b011010 => Self::EretEretaa,
            0b011100 => Self::Pac,
            0b100000 => Self::InstructionAbortLower,
            0b100001 => Self::InstructionAbortSame,
            0b100010 => Self::PcAlignment,
            0b100100 => Self::DataAbortLower,
            0b100101 => Self::DataAbortSame,
            0b100110 => Self::SpAlignment,
            0b101000 => Self::Fp32,
            0b101100 => Self::Fp64,
            0b101111 => Self::SError,
            0b110000 => Self::BreakpointLower,
            0b110001 => Self::BreakpointSame,
            0b110010 => Self::SoftwareStepLower,
            0b110011 => Self::SoftwareStepSame,
            0b110100 => Self::WatchpointLower,
            0b110101 => Self::WatchpointSame,
            0b111000 => Self::Bkpt32,
            0b111100 => Self::Brk64,
            _ => Self::Unknown,
        }
    }
}
