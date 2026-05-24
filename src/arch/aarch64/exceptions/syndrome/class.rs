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
pub enum ExceptionClass {
    Unknown,
    WfeWfi,
    Cp15Mcr,
    Cp15Mcrr,
    Cp14Mcr,
    Cp14Ldc,
    FpAccess,
    Cp14Mrrc,
    BranchTarget,
    IllegalState,
    Svc32,
    Svc64,
    Hvc64,
    Smc64,
    SysReg,
    SveAccess,
    EretEretaa,
    Pac,
    InstructionAbortLower,
    InstructionAbortSame,
    PcAlignment,
    DataAbortLower,
    DataAbortSame,
    SpAlignment,
    Fp32,
    Fp64,
    SError,
    BreakpointLower,
    BreakpointSame,
    SoftwareStepLower,
    SoftwareStepSame,
    WatchpointLower,
    WatchpointSame,
    Bkpt32,
    Brk64,
}
