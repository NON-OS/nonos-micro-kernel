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

pub fn cap_bit(name: &str) -> Option<u64> {
    let bit = match name {
        "CoreExec" => 1,
        "IO" => 2,
        "Network" => 4,
        "IPC" => 8,
        "Memory" => 16,
        "Crypto" => 32,
        "FileSystem" => 64,
        "Hardware" => 128,
        "Debug" => 256,
        "Admin" => 512,
        "RegisterService" => 1024,
        "GraphicsDisplayQuery" => 2048,
        "GraphicsSurfaceCreate" => 4096,
        "GraphicsSurfaceMap" => 8192,
        "GraphicsPresent" => 16384,
        "DeviceEnum" => 32768,
        "Driver" => 65536,
        "Mmio" => 131072,
        "Irq" => 262144,
        "Dma" => 524288,
        "Pio" => 1048576,
        "InputSource" => 2097152,
        "TimeSet" => 4194304,
        // The table stopped here, so a manifest naming any of the following was
        // rejected as an unknown capability and had to state a raw hex mask
        // instead. Values are the kernel's; see `Capability::bit`.
        "SpawnBroker" => 8388608,
        "SpawnWindow" => 16777216,
        "ProcessControl" => 33554432,
        "StoreWrite" => 67108864,
        "EnrolDevRoot" => 134217728,
        "Keyring" => 268435456,
        "Entropy" => 536870912,
        "AppInstall" => 1073741824,
        _ => return None,
    };
    Some(bit)
}
