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

use super::sdt::SdtHeader;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Dmar {
    pub header: SdtHeader,
    pub host_address_width: u8,
    pub flags: u8,
    pub reserved: [u8; 10],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Drhd {
    pub kind: u16,
    pub length: u16,
    pub flags: u8,
    pub reserved: u8,
    pub segment: u16,
    pub register_base_address: u64,
}
