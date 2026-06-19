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

pub const CLASS_DISPLAY: u32 = 0x0030;
pub const PCI_CLASS_DISPLAY: u8 = 0x03;
pub const VENDOR_QEMU_BOCHS: u16 = 0x1234;
pub const DEVICE_BGA: u16 = 0x1111;
pub const FB_BAR: u8 = 0;
pub const REG_BAR: u8 = 2;

pub const DISPI_IOPORT_OFFSET: u32 = 0x500;
pub const DISPI_INDEX_XRES: u32 = 1;
pub const DISPI_INDEX_YRES: u32 = 2;
pub const DISPI_INDEX_BPP: u32 = 3;
pub const DISPI_INDEX_ENABLE: u32 = 4;
pub const DISPI_BPP_32: u16 = 32;
pub const DISPI_ENABLED: u16 = 0x01;
pub const DISPI_LFB_ENABLED: u16 = 0x40;

pub const MODE_WIDTH: u16 = 1024;
pub const MODE_HEIGHT: u16 = 768;
pub const BYTES_PER_PIXEL: u32 = 4;
pub const CLEAR_COLOR: u32 = 0x0010_2a3a;
