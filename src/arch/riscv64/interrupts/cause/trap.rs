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

use super::{ExceptionCode, InterruptCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapCause {
    Exception(ExceptionCode),
    Interrupt(InterruptCode),
}

impl TrapCause {
    pub fn from_scause(scause: usize) -> Self {
        let code = scause & ((1 << 63) - 1);
        if (scause >> 63) != 0 {
            Self::Interrupt(InterruptCode::from(code))
        } else {
            Self::Exception(ExceptionCode::from(code))
        }
    }
}
