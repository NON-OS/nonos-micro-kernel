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
pub enum FaultStatusCode {
    AddressSizeFaultL0,
    AddressSizeFaultL1,
    AddressSizeFaultL2,
    AddressSizeFaultL3,
    TranslationFaultL0,
    TranslationFaultL1,
    TranslationFaultL2,
    TranslationFaultL3,
    AccessFlagFaultL1,
    AccessFlagFaultL2,
    AccessFlagFaultL3,
    PermissionFaultL1,
    PermissionFaultL2,
    PermissionFaultL3,
    SynchronousExternalAbort,
    SynchronousTagCheckFail,
    AlignmentFault,
    TlbConflict,
    Unknown,
}

impl From<u8> for FaultStatusCode {
    fn from(dfsc: u8) -> Self {
        match dfsc {
            0b000000 => Self::AddressSizeFaultL0,
            0b000001 => Self::AddressSizeFaultL1,
            0b000010 => Self::AddressSizeFaultL2,
            0b000011 => Self::AddressSizeFaultL3,
            0b000100 => Self::TranslationFaultL0,
            0b000101 => Self::TranslationFaultL1,
            0b000110 => Self::TranslationFaultL2,
            0b000111 => Self::TranslationFaultL3,
            0b001001 => Self::AccessFlagFaultL1,
            0b001010 => Self::AccessFlagFaultL2,
            0b001011 => Self::AccessFlagFaultL3,
            0b001101 => Self::PermissionFaultL1,
            0b001110 => Self::PermissionFaultL2,
            0b001111 => Self::PermissionFaultL3,
            0b010000 => Self::SynchronousExternalAbort,
            0b010001 => Self::SynchronousTagCheckFail,
            0b100001 => Self::AlignmentFault,
            0b110000 => Self::TlbConflict,
            _ => Self::Unknown,
        }
    }
}
