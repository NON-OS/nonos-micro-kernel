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

use super::super::error::DriverNvmeError;

const E_INVAL: i32 = -22;
const E_IO: i32 = -5;
const E_NXIO: i32 = -6;
const E_NODEV: i32 = -19;
const E_MSGSIZE: i32 = -90;

pub(super) fn lift(status: i32) -> DriverNvmeError {
    match status {
        E_INVAL => DriverNvmeError::InvalidArgument,
        E_IO => DriverNvmeError::DeviceFailure,
        E_NXIO => DriverNvmeError::OutOfRange,
        E_NODEV => DriverNvmeError::Unsupported,
        E_MSGSIZE => DriverNvmeError::OversizedRequest,
        _ => DriverNvmeError::DeviceFailure,
    }
}
