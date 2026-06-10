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

pub const MAX_FIRMWARE_ENTRIES: usize = 64;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FirmwareType {
    Unknown = 0,
    IntelAx200 = 1,
    IntelAx210 = 2,
    Intel8265 = 3,
    Intel9260 = 4,
    Intel7265 = 5,
    IntelAc8260 = 6,
    IntelAc3168 = 7,
    IntelAc9560 = 8,
    IntelAc7260 = 9,
    IntelAc3165 = 10,
    Rtl8821c = 20,
    Rtl8822b = 21,
    Rtl8822c = 22,
    Rtl8723d = 23,
    Rtl8851b = 24,
    Rtl8852a = 25,
    Rtl8852b = 26,
    Rtl8852c = 27,
    Rtl8821a = 28,
    Rtl8723b = 29,
    AtherosAr9271 = 40,
    AtherosAr9170 = 41,
    BroadcomBcm43xx = 50,
    BroadcomBcm4356 = 51,
    MediaTekMt7601u = 60,
    MediaTekMt7921k = 61,
    QualcommAth10k = 70,
    QualcommAth11k = 71,
    QualcommWcn36xx = 72,
    QualcommWcn39xx = 73,
    MarvelMw88w8897 = 80,
    MarvelMw88w8997 = 81,
    RalinRt2870 = 90,
    RalinRt3070 = 91,
    RalinRt5370 = 92,
    AmdGpu = 100,
    NvidiaGpuGt1030 = 101,
    NvidiaGpuGtx1060 = 102,
    NvidiaGpuRtx3070 = 103,
    IntelGpuIgp = 104,
    IntelI219V = 110,
    IntelI225V = 111,
    RealtekRtl8111 = 112,
    RealtekRtl8125 = 113,
    QualcommQca6174 = 114,
}

impl Default for FirmwareType {
    fn default() -> Self {
        FirmwareType::Unknown
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FirmwareEntry {
    pub fw_type: FirmwareType,
    pub ptr: u64,
    pub size: u32,
    pub reserved: u32,
}

impl FirmwareEntry {
    pub const fn empty() -> Self {
        Self { fw_type: FirmwareType::Unknown, ptr: 0, size: 0, reserved: 0 }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FirmwareHandoff {
    pub count: usize,
    pub entries: [FirmwareEntry; MAX_FIRMWARE_ENTRIES],
}

impl FirmwareHandoff {
    pub const fn new() -> Self {
        Self { count: 0, entries: [FirmwareEntry::empty(); MAX_FIRMWARE_ENTRIES] }
    }
}
