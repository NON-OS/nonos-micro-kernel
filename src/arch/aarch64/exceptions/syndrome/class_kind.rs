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

impl ExceptionClass {
    pub fn is_data_abort(&self) -> bool {
        matches!(self, Self::DataAbortLower | Self::DataAbortSame)
    }

    pub fn is_instruction_abort(&self) -> bool {
        matches!(self, Self::InstructionAbortLower | Self::InstructionAbortSame)
    }

    pub fn is_alignment(&self) -> bool {
        matches!(self, Self::PcAlignment | Self::SpAlignment)
    }

    pub fn is_syscall(&self) -> bool {
        matches!(self, Self::Svc32 | Self::Svc64)
    }

    pub fn is_debug(&self) -> bool {
        matches!(
            self,
            Self::BreakpointLower
                | Self::BreakpointSame
                | Self::SoftwareStepLower
                | Self::SoftwareStepSame
                | Self::WatchpointLower
                | Self::WatchpointSame
                | Self::Bkpt32
                | Self::Brk64
        )
    }
}
