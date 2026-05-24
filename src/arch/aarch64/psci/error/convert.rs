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

use super::kind::PsciError;

impl PsciError {
    pub fn from_ret(ret: i32) -> Result<(), Self> {
        match ret {
            0 => Ok(()),
            -1 => Err(Self::NotSupported),
            -2 => Err(Self::InvalidParams),
            -3 => Err(Self::Denied),
            -4 => Err(Self::AlreadyOn),
            -5 => Err(Self::OnPending),
            -6 => Err(Self::InternalFailure),
            -7 => Err(Self::NotPresent),
            -8 => Err(Self::Disabled),
            -9 => Err(Self::InvalidAddress),
            other => Err(Self::Unknown(other)),
        }
    }

    pub fn code(&self) -> i32 {
        match self {
            Self::Success => 0,
            Self::NotSupported => -1,
            Self::InvalidParams => -2,
            Self::Denied => -3,
            Self::AlreadyOn => -4,
            Self::OnPending => -5,
            Self::InternalFailure => -6,
            Self::NotPresent => -7,
            Self::Disabled => -8,
            Self::InvalidAddress => -9,
            Self::Unknown(code) => *code,
        }
    }
}
