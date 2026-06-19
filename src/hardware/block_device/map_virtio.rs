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
use crate::hardware::virtio_blk_capsule::DriverBlkError;

pub(super) fn map_virtio_error(e: DriverBlkError) -> BlockDeviceError {
    match e {
        DriverBlkError::Dead => BlockDeviceError::Dead,
        DriverBlkError::Stale => BlockDeviceError::Stale,
        DriverBlkError::AccessDenied => BlockDeviceError::AccessDenied,
        DriverBlkError::InvalidArgument => BlockDeviceError::InvalidArgument,
        DriverBlkError::OversizedRequest => BlockDeviceError::OversizedRequest,
        DriverBlkError::OutOfRange => BlockDeviceError::OutOfRange,
        DriverBlkError::DeviceFailure => BlockDeviceError::DeviceFailure,
        DriverBlkError::Unsupported => BlockDeviceError::Unsupported,
        DriverBlkError::NoCallerPid => BlockDeviceError::NoCallerPid,
        DriverBlkError::TransportFailure => BlockDeviceError::TransportFailure,
        DriverBlkError::ProtocolMismatch => BlockDeviceError::ProtocolMismatch,
    }
}
