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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HdaError {
    DeviceNotFound,
    BrokerCallFailed(i64),
    ControllerResetTimeout,
    ImmediateCommandBusy,
    ImmediateResponseTimeout,
    UnsupportedController,
    VerbTimeout,
}

pub type HdaResult<T> = Result<T, HdaError>;

pub fn exit_code(e: HdaError) -> i32 {
    match e {
        HdaError::DeviceNotFound => 2,
        HdaError::BrokerCallFailed(_) => 3,
        HdaError::ControllerResetTimeout => 4,
        HdaError::ImmediateCommandBusy => 5,
        HdaError::ImmediateResponseTimeout => 6,
        HdaError::UnsupportedController => 7,
        HdaError::VerbTimeout => 8,
    }
}
