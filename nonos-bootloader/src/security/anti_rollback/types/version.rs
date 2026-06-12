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

#[derive(Clone, Copy)]
pub struct VersionState {
    pub kernel_version: u64,
    pub bootloader_version: u64,
    pub minimum_kernel: u64,
    pub minimum_bootloader: u64,
    pub last_boot_timestamp: u64,
    pub boot_count: u64,
}

impl VersionState {
    pub const fn new() -> Self {
        Self {
            kernel_version: 0,
            bootloader_version: 0,
            minimum_kernel: 0,
            minimum_bootloader: 0,
            last_boot_timestamp: 0,
            boot_count: 0,
        }
    }
}
