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
use crate::hardware::ahci_capsule::DriverAhciError;

pub(super) fn map_ahci_error(e: DriverAhciError) -> BlockDeviceError {
    match e {
        DriverAhciError::Dead => BlockDeviceError::Dead,
        DriverAhciError::Stale => BlockDeviceError::Stale,
        DriverAhciError::AccessDenied => BlockDeviceError::AccessDenied,
        DriverAhciError::InvalidArgument => BlockDeviceError::InvalidArgument,
        DriverAhciError::OversizedRequest => BlockDeviceError::OversizedRequest,
        DriverAhciError::OutOfRange => BlockDeviceError::OutOfRange,
        DriverAhciError::DeviceFailure => BlockDeviceError::DeviceFailure,
        DriverAhciError::Unsupported => BlockDeviceError::Unsupported,
        DriverAhciError::NoCallerPid => BlockDeviceError::NoCallerPid,
        DriverAhciError::TransportFailure => BlockDeviceError::TransportFailure,
        DriverAhciError::ProtocolMismatch => BlockDeviceError::ProtocolMismatch,
    }
}
