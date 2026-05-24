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
use super::xhci_error::XhciError;
const ENODEV: i32 = -19;
const EIO: i32 = -5;
const ENOTSUP: i32 = -95;
const ETIMEDOUT: i32 = -110;
const EAGAIN: i32 = -11;
const EREMOTEIO: i32 = -121;
pub fn errno_value(e: XhciError) -> i32 {
    match e {
        XhciError::DeviceNotFound => ENODEV,
        XhciError::BrokerCallFailed(_) => EIO,
        XhciError::ControllerUnsupported => ENOTSUP,
        XhciError::ResetTimeout
        | XhciError::ControllerNotReadyTimeout
        | XhciError::StartTimeout
        | XhciError::HaltTimeout
        | XhciError::CommandCompletionTimeout => ETIMEDOUT,
        XhciError::CommandRingFull => EAGAIN,
        XhciError::TransferRingFull => EAGAIN,
        XhciError::NoDeviceOnPort => ENODEV,
        XhciError::PortResetTimeout | XhciError::TransferCompletionTimeout => ETIMEDOUT,
        XhciError::CommandCompletionFailed(_) => EREMOTEIO,
        XhciError::UnexpectedCompletionSlot => EREMOTEIO,
        XhciError::TransferCompletionFailed(_) => EREMOTEIO,
    }
}