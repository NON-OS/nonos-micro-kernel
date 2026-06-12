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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PcrIndex {
    Firmware = 0,
    FirmwareConfig = 1,
    OptionRom = 2,
    BootConfig = 3,
    Mbr = 4,
    GptPartition = 5,
    VendorSpecific = 6,
    SecureBootState = 7,
    Bootloader = 8,
    Kernel = 9,
    ZkProof = 10,
    BootAudit = 11,
}
