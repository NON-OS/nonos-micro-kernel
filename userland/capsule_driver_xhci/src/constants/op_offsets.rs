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
pub const USBCMD: u64 = 0x00;
pub const USBSTS: u64 = 0x04;
pub const CRCR_LO: u64 = 0x18;
pub const DCBAAP_LO: u64 = 0x30;
pub const CONFIG: u64 = 0x38;
pub const PORTSC_BASE: u64 = 0x400;
pub const PORT_REG_STRIDE: u64 = 0x10;
