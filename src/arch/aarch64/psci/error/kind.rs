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
pub enum PsciError {
    Success,
    NotSupported,
    InvalidParams,
    Denied,
    AlreadyOn,
    OnPending,
    InternalFailure,
    NotPresent,
    Disabled,
    InvalidAddress,
    Unknown(i32),
}

impl PsciError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::NotSupported => "not supported",
            Self::InvalidParams => "invalid parameters",
            Self::Denied => "denied",
            Self::AlreadyOn => "already on",
            Self::OnPending => "on pending",
            Self::InternalFailure => "internal failure",
            Self::NotPresent => "not present",
            Self::Disabled => "disabled",
            Self::InvalidAddress => "invalid address",
            Self::Unknown(_) => "unknown error",
        }
    }
}
