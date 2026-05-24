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

use super::kind::SbiError;

impl SbiError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failed => "failed",
            Self::NotSupported => "not supported",
            Self::InvalidParam => "invalid parameter",
            Self::Denied => "denied",
            Self::InvalidAddress => "invalid address",
            Self::AlreadyAvailable => "already available",
            Self::AlreadyStarted => "already started",
            Self::AlreadyStopped => "already stopped",
            Self::Unknown(_) => "unknown error",
        }
    }

    pub fn code(&self) -> isize {
        match self {
            Self::Success => 0,
            Self::Failed => -1,
            Self::NotSupported => -2,
            Self::InvalidParam => -3,
            Self::Denied => -4,
            Self::InvalidAddress => -5,
            Self::AlreadyAvailable => -6,
            Self::AlreadyStarted => -7,
            Self::AlreadyStopped => -8,
            Self::Unknown(code) => *code,
        }
    }
}
