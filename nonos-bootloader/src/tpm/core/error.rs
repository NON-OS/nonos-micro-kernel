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
pub enum TmpError {
    DeviceNotFound,
    LocalityTimeout,
    CommandTimeout,
    InvalidResponse,
    BadParameter,
    AccessDenied,
    ResourceUnavailable,
    CommunicationFailure,
    InvalidHandle,
    SessionExpired,
    PolicyFailure,
}

pub type TmpResult<T> = Result<T, TmpError>;

impl core::fmt::Display for TmpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DeviceNotFound => write!(f, "device not found"),
            Self::LocalityTimeout => write!(f, "locality timeout"),
            Self::CommandTimeout => write!(f, "command timeout"),
            Self::InvalidResponse => write!(f, "invalid response"),
            Self::BadParameter => write!(f, "bad parameter"),
            Self::AccessDenied => write!(f, "access denied"),
            Self::ResourceUnavailable => write!(f, "resource unavailable"),
            Self::CommunicationFailure => write!(f, "communication failure"),
            Self::InvalidHandle => write!(f, "invalid handle"),
            Self::SessionExpired => write!(f, "session expired"),
            Self::PolicyFailure => write!(f, "policy failure"),
        }
    }
}
