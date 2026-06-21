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

use super::family::HardwareFamily::{self, *};
use super::state::SupportState::{self, *};

pub fn support_state(family: HardwareFamily) -> SupportState {
    match family {
        StorageVirtioBlk | StorageUsbMsc | StorageAhci | StorageNvme => DataPath,
        NetworkVirtio | NetworkE1000 | NetworkRtl8139 | NetworkRtl8169 => DataPath,
        DisplayGopFramebuffer | DisplayVirtioGpu | DisplayBga => DataPath,
        UsbXhci | InputPs2 | InputUsbHid => DataPath,
        NetworkIwlwifi | AudioHda | InputI2cHid | SerialI2c => ControllerStatus,
        DisplayNativeIntel | DisplayNativeAmd | DisplayNativeNvidia => EnumerateOnly,
        UsbEhci | UsbOhci | UsbUhci | SerialSpi | BridgePci | SystemPeripheral | Unknown => {
            EnumerateOnly
        }
    }
}
