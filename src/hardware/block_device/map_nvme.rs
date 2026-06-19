// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::BlockDeviceError;
use crate::hardware::nvme_capsule::DriverNvmeError;

pub(super) fn map_nvme_error(e: DriverNvmeError) -> BlockDeviceError {
    match e {
        DriverNvmeError::Dead => BlockDeviceError::Dead,
        DriverNvmeError::Stale => BlockDeviceError::Stale,
        DriverNvmeError::AccessDenied => BlockDeviceError::AccessDenied,
        DriverNvmeError::InvalidArgument => BlockDeviceError::InvalidArgument,
        DriverNvmeError::OversizedRequest => BlockDeviceError::OversizedRequest,
        DriverNvmeError::OutOfRange => BlockDeviceError::OutOfRange,
        DriverNvmeError::DeviceFailure => BlockDeviceError::DeviceFailure,
        DriverNvmeError::Unsupported => BlockDeviceError::Unsupported,
        DriverNvmeError::NoCallerPid => BlockDeviceError::NoCallerPid,
        DriverNvmeError::TransportFailure => BlockDeviceError::TransportFailure,
        DriverNvmeError::ProtocolMismatch => BlockDeviceError::ProtocolMismatch,
    }
}
